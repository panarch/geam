use super::RuntimeGraphState;
use super::{BlockEnvironment, CompletedGraph, GraphPosition, RetainedValues};
use crate::StringValue;
use crate::plan::execution::constant::{ConstantId, ConstantValue, ProfiledConstantProgram};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionNeverFunctionBody, FunctionBodyOwner,
    FunctionExit,
};
use crate::plan::execution::graph::BlockGraphView;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedFunctionValue,
    EvaluatedValue,
};
use crate::runtime::function::EntryTarget;
use crate::runtime::state::list::{
    BitArrayListValueId, BoolListValueId, CustomListValueId, ExternalListValueId, FloatListValueId,
    FunctionListValueId, IntListValueId, ListListValueId, NilListValueId, ParameterListListValueId,
    ParameterListValueId, StringListValueId, TupleListValueId, UtfCodepointListValueId,
};
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};
use num_bigint::BigInt;
use std::marker::PhantomData;

pub(in crate::runtime) struct Execution<'plan, Plan: ExecutableRuntimePlan> {
    active: Activation<'plan, Plan>,
}

pub(in crate::runtime) enum Progress<'plan, Plan: ExecutableRuntimePlan + 'plan> {
    Continue(Execution<'plan, Plan>),
    Host(Plan::HostInvocation<'plan, Execution<'plan, Plan>>),
    Complete(CompletedGraph),
}

pub(super) enum Activation<'plan, Plan: ExecutableRuntimePlan + 'plan> {
    Graph(Frame<'plan, Plan>),
    Host(Plan::HostInvocation<'plan, Activation<'plan, Plan>>),
    Return(Return<'plan, Plan>),
    Complete(CompletedGraph),
}

pub(super) struct Frame<'plan, Plan: ExecutableRuntimePlan> {
    pub(super) graph: BlockGraphView<'plan, RuntimeGraph<Plan>>,
    pub(super) position: GraphPosition,
    exit: Box<dyn GraphExit<'plan, Plan> + 'plan>,
}

trait GraphExit<'plan, Plan: ExecutableRuntimePlan>: Send {
    fn exit(
        self: Box<Self>,
        completed: CompletedGraph,
        returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>>;
}

struct RootExit;

impl<'plan, Plan: ExecutableRuntimePlan> GraphExit<'plan, Plan> for RootExit {
    fn exit(
        self: Box<Self>,
        completed: CompletedGraph,
        _returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        Ok(Activation::Complete(completed))
    }
}

struct FunctionContinuation<'plan, Plan, Id, Value, Map>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan>,
{
    plan: &'plan Plan,
    id: Id,
    body: &'plan Id::Body,
    destination: Destination<Value>,
    map: Map,
}

struct ConstantContinuation<'plan, Plan: ExecutableRuntimePlan, Local: 'static, Value, Map> {
    constant: &'plan ProfiledConstantProgram<Local, RuntimeGraph<Plan>>,
    destination: Destination<Value>,
    map: Map,
}

struct NeverContinuation<'plan, Plan: ExecutableRuntimePlan> {
    plan: &'plan Plan,
    body: &'plan ExecutionNeverFunctionBody<Plan::Profile>,
}

type Return<'plan, Plan> =
    Box<dyn FnOnce(&mut Returns<'plan, Plan>) -> Activation<'plan, Plan> + Send + 'plan>;

pub(super) struct Destination<Value> {
    domain: Option<crate::runtime::captures::ExecutionDomain>,
    index: usize,
    value: PhantomData<fn(Value)>,
}

pub(in crate::runtime) struct Returns<'plan, Plan: ExecutableRuntimePlan> {
    domain: Option<crate::runtime::captures::ExecutionDomain>,
    ints: Vec<Frame<'plan, Plan>>,
    floats: Vec<Frame<'plan, Plan>>,
    strings: Vec<Frame<'plan, Plan>>,
    bit_arrays: Vec<Frame<'plan, Plan>>,
    utf_codepoints: Vec<Frame<'plan, Plan>>,
    customs: Vec<Frame<'plan, Plan>>,
    externals: Vec<Frame<'plan, Plan>>,
    bools: Vec<Frame<'plan, Plan>>,
    nils: Vec<Frame<'plan, Plan>>,
    tuples: Vec<Frame<'plan, Plan>>,
    functions: Vec<Frame<'plan, Plan>>,
    parameter_lists: Vec<Frame<'plan, Plan>>,
    parameter_list_lists: Vec<Frame<'plan, Plan>>,
    int_lists: Vec<Frame<'plan, Plan>>,
    string_lists: Vec<Frame<'plan, Plan>>,
    bit_array_lists: Vec<Frame<'plan, Plan>>,
    utf_codepoint_lists: Vec<Frame<'plan, Plan>>,
    custom_lists: Vec<Frame<'plan, Plan>>,
    external_lists: Vec<Frame<'plan, Plan>>,
    float_lists: Vec<Frame<'plan, Plan>>,
    bool_lists: Vec<Frame<'plan, Plan>>,
    nil_lists: Vec<Frame<'plan, Plan>>,
    tuple_lists: Vec<Frame<'plan, Plan>>,
    list_lists: Vec<Frame<'plan, Plan>>,
    function_lists: Vec<Frame<'plan, Plan>>,
}

pub(super) trait ReturnValue: Send + 'static {
    fn frames<'stack, 'plan, Plan: ExecutableRuntimePlan>(
        returns: &'stack mut Returns<'plan, Plan>,
    ) -> &'stack mut Vec<Frame<'plan, Plan>>;

    fn push(self, environment: &mut BlockEnvironment);
}

impl<'plan, Plan: ExecutableRuntimePlan> Execution<'plan, Plan> {
    pub(in crate::runtime) fn new(
        graph: BlockGraphView<'plan, RuntimeGraph<Plan>>,
        inputs: RetainedValues,
    ) -> Self {
        Self {
            active: Activation::Graph(Frame {
                graph,
                position: GraphPosition::new(graph.entry(), inputs),
                exit: Box::new(RootExit),
            }),
        }
    }

    pub(in crate::runtime) fn advance(
        self,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = crate::ExecutionError>,
        returns: &mut Returns<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Progress<'plan, Plan>> {
        returns.domain = Some(state.captures().domain());
        let active = match self.active {
            // The caller charged this activation; only additional steps consume remaining budget.
            Activation::Graph(mut frame) => loop {
                let block = frame.graph.block(frame.position.block);
                let active = if frame.position.instruction < block.instructions().len() {
                    super::instruction::advance(plan, state, frame, returns, remaining)?
                } else {
                    use super::terminator::{GraphAction, NeverCall, terminator_action};

                    match terminator_action(
                        plan,
                        state,
                        frame.position.environment,
                        block.terminator(),
                    )? {
                        GraphAction::Continue { block, inputs } => {
                            frame.position = GraphPosition::new(block, inputs);
                            if *remaining > 0 {
                                *remaining -= 1;
                                continue;
                            }
                            break Activation::Graph(frame);
                        }
                        GraphAction::Exit { exit, environment } => frame
                            .exit
                            .exit(CompletedGraph { exit, environment }, returns)?,
                        GraphAction::NeverCall {
                            function,
                            mut inputs,
                            site,
                        } => {
                            drop(frame.exit);
                            let function = match function {
                                NeverCall::Direct(function) => function,
                                NeverCall::Value(function) => {
                                    inputs.append_captures(function.capture_frame());
                                    function.runtime_id()
                                }
                            };
                            if let Some(cancelled) = plan
                                .reject_foreign_callable(&inputs, Some(state.captures().domain()))
                            {
                                Activation::Host(cancelled)
                            } else {
                                enter_never(plan, function, HostCallOrigin::source(site), inputs)
                            }
                        }
                    }
                };
                match active {
                    Activation::Graph(next) if *remaining > 0 => {
                        *remaining -= 1;
                        frame = next;
                    }
                    active => break active,
                }
            },
            Activation::Host(invoke) => {
                return Ok(Progress::Host(Plan::map_host(invoke, |active| {
                    Ok(Self { active })
                })));
            }
            Activation::Return(resume) => resume(returns),
            Activation::Complete(completed) => return Ok(Progress::Complete(completed)),
        };
        Ok(Progress::Continue(Self { active }))
    }
}

impl<'plan, Plan: ExecutableRuntimePlan> Frame<'plan, Plan> {
    pub(super) fn store<Value: ReturnValue>(mut self, value: Value) -> Activation<'plan, Plan> {
        value.push(&mut self.position.environment);
        Activation::Graph(self)
    }
}

impl<'plan, Plan: ExecutableRuntimePlan> Returns<'plan, Plan> {
    pub(in crate::runtime) fn new() -> Self {
        Self {
            domain: None,
            ints: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            customs: Vec::new(),
            externals: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            tuples: Vec::new(),
            functions: Vec::new(),
            parameter_lists: Vec::new(),
            parameter_list_lists: Vec::new(),
            int_lists: Vec::new(),
            string_lists: Vec::new(),
            bit_array_lists: Vec::new(),
            utf_codepoint_lists: Vec::new(),
            custom_lists: Vec::new(),
            external_lists: Vec::new(),
            float_lists: Vec::new(),
            bool_lists: Vec::new(),
            nil_lists: Vec::new(),
            tuple_lists: Vec::new(),
            list_lists: Vec::new(),
            function_lists: Vec::new(),
        }
    }

    pub(super) fn suspend<Value: ReturnValue>(
        &mut self,
        frame: Frame<'plan, Plan>,
    ) -> Destination<Value> {
        let frames = Value::frames(self);
        let index = frames.len();
        frames.push(frame);
        Destination {
            domain: self.domain,
            index,
            value: PhantomData,
        }
    }
}

impl<Value: ReturnValue> Destination<Value> {
    fn resume<'plan, Plan: ExecutableRuntimePlan>(
        self,
        returns: &mut Returns<'plan, Plan>,
        value: Value,
    ) -> Activation<'plan, Plan> {
        // Only this active call chain can create or consume its typed destinations.
        Value::frames(returns).swap_remove(self.index).store(value)
    }
}

pub(super) fn enter_function<'plan, Plan, Id, Value>(
    plan: &'plan Plan,
    id: Id,
    origin: HostCallOrigin,
    inputs: RetainedValues,
    destination: Destination<Value>,
    map: impl FnOnce(
        <<Id::Body as FunctionBodyOwner>::Return as super::GraphValue>::Evaluated,
    ) -> ExecutionResult<Value>
    + Send
    + 'plan,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Id::Body: 'plan,
    Value: ReturnValue,
{
    if let Some(cancelled) = plan.reject_foreign_callable(&inputs, destination.domain) {
        return Activation::Host(cancelled);
    }
    match id.entry(plan) {
        ExecutionFunctionRef::Graph(function) => Box::new(FunctionContinuation {
            plan,
            id,
            body: function.body(),
            destination,
            map,
        })
        .enter(inputs),
        ExecutionFunctionRef::Host(target) => Activation::Host(Plan::map_host(
            Id::prepare_host(plan, origin, target, inputs),
            move |value| {
                let value = map(value)?;
                Ok(Activation::Return(Box::new(move |returns| {
                    destination.resume(returns, value)
                })))
            },
        )),
    }
}

impl<'plan, Plan, Id, Value, Map> FunctionContinuation<'plan, Plan, Id, Value, Map>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Value: ReturnValue,
    Map: FnOnce(
            <<Id::Body as FunctionBodyOwner>::Return as super::GraphValue>::Evaluated,
        ) -> ExecutionResult<Value>
        + Send
        + 'plan,
{
    fn enter(self: Box<Self>, inputs: RetainedValues) -> Activation<'plan, Plan> {
        let graph = self.body.function_body().block_graph().as_view();
        Activation::Graph(Frame {
            graph,
            position: GraphPosition::new(graph.entry(), inputs),
            exit: self,
        })
    }
}

impl<'plan, Plan, Id, Value, Map> GraphExit<'plan, Plan>
    for FunctionContinuation<'plan, Plan, Id, Value, Map>
where
    Plan: ExecutableRuntimePlan,
    Id: EntryTarget<Plan> + 'plan,
    Value: ReturnValue,
    Map: FnOnce(
            <<Id::Body as FunctionBodyOwner>::Return as super::GraphValue>::Evaluated,
        ) -> ExecutionResult<Value>
        + Send
        + 'plan,
{
    fn exit(
        mut self: Box<Self>,
        completed: CompletedGraph,
        returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        match self.body.function_body().exit(completed.exit()) {
            FunctionExit::Return(value) => {
                let value = (self.map)(completed.into_value(value))?;
                Ok(self.destination.resume(returns, value))
            }
            FunctionExit::TailCall {
                function, transfer, ..
            } => {
                let (id, origin) = self.id.next(function);
                let inputs = completed.into_retained(transfer);
                // A direct source tail transfers locals within this activation.
                // Callable-value entry and capture-domain checks stay in enter_function.
                match id.entry(self.plan) {
                    ExecutionFunctionRef::Graph(function) => {
                        self.id = id;
                        self.body = function.body();
                        Ok(self.enter(inputs))
                    }
                    ExecutionFunctionRef::Host(target) => {
                        let Self {
                            plan,
                            destination,
                            map,
                            ..
                        } = *self;
                        Ok(Activation::Host(Plan::map_host(
                            Id::prepare_host(plan, origin, target, inputs),
                            move |value| {
                                let value = map(value)?;
                                Ok(Activation::Return(Box::new(move |returns| {
                                    destination.resume(returns, value)
                                })))
                            },
                        )))
                    }
                }
            }
        }
    }
}

pub(super) fn enter_constant<'plan, Plan, Local, Value>(
    plan: &'plan Plan,
    id: ConstantId<Local>,
    destination: Destination<Value>,
    map: impl FnOnce(Local::Evaluated) -> Value + Send + 'plan,
) -> Activation<'plan, Plan>
where
    Plan: ExecutableRuntimePlan,
    Local: ConstantValue + super::GraphValue + Sync + 'plan,
    Value: ReturnValue,
{
    let constant = plan.constant(id);
    let graph = constant.block_graph().as_view();
    Activation::Graph(Frame {
        graph,
        position: GraphPosition::new(graph.entry(), RetainedValues::empty()),
        exit: Box::new(ConstantContinuation::<Plan, _, _, _> {
            constant,
            destination,
            map,
        }),
    })
}

impl<'plan, Plan, Local, Value, Map> GraphExit<'plan, Plan>
    for ConstantContinuation<'plan, Plan, Local, Value, Map>
where
    Plan: ExecutableRuntimePlan,
    Local: ConstantValue + super::GraphValue + Sync + 'plan,
    Value: ReturnValue,
    Map: FnOnce(Local::Evaluated) -> Value + Send + 'plan,
{
    fn exit(
        self: Box<Self>,
        completed: CompletedGraph,
        returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        let local = self.constant.return_(completed.exit());
        let value = (self.map)(completed.into_value(local));
        Ok(self.destination.resume(returns, value))
    }
}

fn enter_never<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    id: crate::plan::execution::function::NeverFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> Activation<'plan, Plan> {
    match plan.never_function(id).as_ref() {
        ExecutionFunctionRef::Graph(function) => Box::new(NeverContinuation {
            plan,
            body: function.body(),
        })
        .enter(inputs),
        ExecutionFunctionRef::Host(target) => Activation::Host(Plan::map_host(
            plan.prepare_host_never(origin, target, inputs),
            |never| match never {},
        )),
    }
}

impl<'plan, Plan: ExecutableRuntimePlan> NeverContinuation<'plan, Plan> {
    fn enter(self: Box<Self>, inputs: RetainedValues) -> Activation<'plan, Plan> {
        let graph = self.body.function_body().block_graph().as_view();
        Activation::Graph(Frame {
            graph,
            position: GraphPosition::new(graph.entry(), inputs),
            exit: self,
        })
    }
}

impl<'plan, Plan: ExecutableRuntimePlan> GraphExit<'plan, Plan> for NeverContinuation<'plan, Plan> {
    fn exit(
        mut self: Box<Self>,
        completed: CompletedGraph,
        _returns: &mut Returns<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        match self.body.function_body().exit(completed.exit()) {
            FunctionExit::Return(never) => match *never {},
            FunctionExit::TailCall {
                function, transfer, ..
            } => {
                let inputs = completed.into_retained(transfer);
                let origin = HostCallOrigin::source(function.site().clone());
                match self.plan.never_function(*function.function()).as_ref() {
                    ExecutionFunctionRef::Graph(function) => {
                        self.body = function.body();
                        Ok(self.enter(inputs))
                    }
                    ExecutionFunctionRef::Host(target) => Ok(Activation::Host(Plan::map_host(
                        self.plan.prepare_host_never(origin, target, inputs),
                        |never| match never {},
                    ))),
                }
            }
        }
    }
}

macro_rules! return_value {
    ($value:ty, $frames:ident, $push:ident) => {
        impl ReturnValue for $value {
            fn frames<'stack, 'plan, Plan: ExecutableRuntimePlan>(
                returns: &'stack mut Returns<'plan, Plan>,
            ) -> &'stack mut Vec<Frame<'plan, Plan>> {
                &mut returns.$frames
            }

            fn push(self, environment: &mut BlockEnvironment) {
                environment.$push(self);
            }
        }
    };
}

return_value!(BigInt, ints, push_int);
return_value!(f64, floats, push_float);
return_value!(StringValue, strings, push_string);
return_value!(EvaluatedBitArray, bit_arrays, push_bit_array);
return_value!(char, utf_codepoints, push_utf_codepoint);
return_value!(EvaluatedCustomValue, customs, push_custom);
return_value!(EvaluatedExternalValue, externals, push_external);
return_value!(bool, bools, push_bool);
return_value!(Vec<EvaluatedValue>, tuples, push_tuple);
return_value!(EvaluatedFunctionValue, functions, push_function_value);
return_value!(ParameterListValueId, parameter_lists, push_parameter_list);
return_value!(
    ParameterListListValueId,
    parameter_list_lists,
    push_parameter_list_list
);
return_value!(IntListValueId, int_lists, push_int_list);
return_value!(StringListValueId, string_lists, push_string_list);
return_value!(BitArrayListValueId, bit_array_lists, push_bit_array_list);
return_value!(
    UtfCodepointListValueId,
    utf_codepoint_lists,
    push_utf_codepoint_list
);
return_value!(CustomListValueId, custom_lists, push_custom_list);
return_value!(ExternalListValueId, external_lists, push_external_list);
return_value!(FloatListValueId, float_lists, push_float_list);
return_value!(BoolListValueId, bool_lists, push_bool_list);
return_value!(NilListValueId, nil_lists, push_nil_list);
return_value!(TupleListValueId, tuple_lists, push_tuple_list);
return_value!(ListListValueId, list_lists, push_list_list);
return_value!(FunctionListValueId, function_lists, push_function_list);

impl ReturnValue for () {
    fn frames<'stack, 'plan, Plan: ExecutableRuntimePlan>(
        returns: &'stack mut Returns<'plan, Plan>,
    ) -> &'stack mut Vec<Frame<'plan, Plan>> {
        &mut returns.nils
    }

    fn push(self, environment: &mut BlockEnvironment) {
        environment.push_nil();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Activation, Execution, Frame, GraphPosition, Progress, Returns, RootExit, enter_function,
        enter_never,
    };
    use crate::ExecutionPlan;
    use crate::plan::execution::function::{FunctionExit, IntFunctionId};
    use crate::runtime::graph::{CompletedGraph, RetainedValues};
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{EvaluatedValue, HostCallOrigin, RuntimeListStorage, Value};
    use num_bigint::BigInt;
    use std::collections::BTreeSet;
    use std::ptr;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn returned_int(plan: &ExecutionPlan, id: IntFunctionId, completed: CompletedGraph) -> BigInt {
        let body = plan.int_function(id).body();
        match body.exit(completed.exit()) {
            FunctionExit::Return(value) => completed.into_value(value),
            FunctionExit::TailCall { .. } => {
                panic!("fixture root returns rather than tail-calling")
            }
        }
    }

    fn continuing(progress: Progress<'_, ExecutionPlan>) -> Execution<'_, ExecutionPlan> {
        match progress {
            Progress::Continue(next) => next,
            Progress::Complete(_) => panic!("fixture graph must still be running"),
            Progress::Host(invoke) => match invoke {},
        }
    }

    fn active_frame<'run, 'plan>(
        execution: &'run Execution<'plan, ExecutionPlan>,
    ) -> &'run Frame<'plan, ExecutionPlan> {
        match &execution.active {
            Activation::Graph(frame) => frame,
            _ => panic!("fixture activation must be a graph"),
        }
    }

    fn completed(progress: Progress<'_, ExecutionPlan>) -> CompletedGraph {
        match progress {
            Progress::Complete(completed) => completed,
            Progress::Continue(_) => panic!("fixture graph must have completed"),
            Progress::Host(invoke) => match invoke {},
        }
    }

    fn complete_int_graph(plan: &ExecutionPlan) -> CompletedGraph {
        let body = plan.int_function(IntFunctionId(0)).body();
        let mut execution = Execution::new(body.block_graph().as_view(), RetainedValues::empty());
        let mut returns = Returns::new();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        loop {
            match execution
                .advance(plan, &mut state, &mut returns, &mut 0)
                .unwrap()
            {
                Progress::Continue(next) => execution = next,
                Progress::Complete(completed) => return completed,
                Progress::Host(invoke) => match invoke {},
            }
        }
    }

    #[test]
    fn instruction_runs_preserve_typed_values_at_every_budget_boundary() {
        let plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let number = 20 + 21
  let values = [number, 1]
  let fields = #(number, values, fn(x) { x + number })
  fields.0 + 1
}
"#,
        );
        let graph = plan
            .int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .as_view();
        let instructions = graph.block(graph.entry()).instructions();
        assert!(instructions.len() > 3);
        for budget in 1..=instructions.len() {
            let mut returns = Returns::new();
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            let mut remaining = budget - 1;
            let execution = continuing(
                Execution::new(graph, RetainedValues::empty())
                    .advance(&plan, &mut state, &mut returns, &mut remaining)
                    .unwrap(),
            );
            assert_eq!(remaining, 0);
            let frame = active_frame(&execution);
            assert_eq!(frame.position.block, graph.entry());
            assert_eq!(frame.position.instruction, budget);
            // Resume the stored prefix to the end, without charging its terminator.
            let execution = if budget == instructions.len() {
                execution
            } else {
                continuing(
                    execution
                        .advance(
                            &plan,
                            &mut state,
                            &mut returns,
                            &mut (instructions.len() - budget - 1),
                        )
                        .unwrap(),
                )
            };
            let frame = active_frame(&execution);
            assert_eq!(frame.position.instruction, instructions.len());
            assert_eq!(
                frame
                    .position
                    .environment
                    .value(instructions.last().unwrap().output().local()),
                EvaluatedValue::Int(42.into()),
            );
            let execution = continuing(
                execution
                    .advance(&plan, &mut state, &mut returns, &mut 0)
                    .unwrap(),
            );
            let completed = completed(
                execution
                    .advance(&plan, &mut state, &mut returns, &mut 0)
                    .unwrap(),
            );
            assert_eq!(returned_int(&plan, IntFunctionId(0), completed), 42.into());
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn empty_and_single_instruction_blocks_keep_separate_termination_steps() {
        for (source, id, instruction_count, inputs) in [
            (
                "fn identity(value: Int) { value } pub fn main() { identity(42) }",
                IntFunctionId(1),
                0,
                vec![EvaluatedValue::Int(42.into())],
            ),
            ("pub fn main() { 42 }", IntFunctionId(0), 1, Vec::new()),
        ] {
            let plan = crate::runtime::plan_src(source);
            let graph = plan.int_function(id).body().block_graph().as_view();
            assert_eq!(
                graph.block(graph.entry()).instructions().len(),
                instruction_count
            );
            let mut retained = RetainedValues::empty();
            for value in inputs {
                retained.push_evaluated(value);
            }
            let mut execution = Execution::new(graph, retained);
            let mut returns = Returns::new();
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            for _ in 0..instruction_count {
                execution = continuing(
                    execution
                        .advance(&plan, &mut state, &mut returns, &mut 0)
                        .unwrap(),
                );
                assert_eq!(
                    active_frame(&execution).position.instruction,
                    instruction_count
                );
            }
            execution = continuing(
                execution
                    .advance(&plan, &mut state, &mut returns, &mut 0)
                    .unwrap(),
            );
            let completed = completed(
                execution
                    .advance(&plan, &mut state, &mut returns, &mut 0)
                    .unwrap(),
            );
            assert_eq!(returned_int(&plan, id, completed), 42.into());
            assert!(echo.is_empty());
        }
    }

    #[test]
    #[should_panic(expected = "fixture root returns rather than tail-calling")]
    fn returned_int_rejects_a_source_tail_call() {
        let plan = crate::runtime::plan_src("fn answer() { 42 } pub fn main() { answer() }");
        returned_int(&plan, IntFunctionId(0), complete_int_graph(&plan));
    }

    #[test]
    #[should_panic(expected = "fixture graph must still be running")]
    fn continuing_rejects_a_completed_source_graph() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        continuing(Progress::Complete(complete_int_graph(&plan)));
    }

    #[test]
    #[should_panic(expected = "fixture activation must be a graph")]
    fn active_frame_rejects_a_completed_activation() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        active_frame(&Execution {
            active: Activation::Complete(complete_int_graph(&plan)),
        });
    }

    #[test]
    #[should_panic(expected = "fixture graph must have completed")]
    fn completed_rejects_a_running_source_graph() {
        let plan = crate::runtime::plan_src("pub fn main() { 42 }");
        let graph = plan
            .int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .as_view();
        completed(Progress::Continue(Execution::new(
            graph,
            RetainedValues::empty(),
        )));
    }

    #[test]
    fn source_tail_calls_keep_one_exit_owner_across_self_and_mutual_recursion() {
        for (source, expected_bodies) in [
            (
                r#"
fn walk(n) { case n { 0 -> 41 _ -> walk(n - 1) } }
pub fn main() { walk(20) + 1 }
"#,
                1,
            ),
            (
                r#"
fn left(n) { case n { 0 -> 41 _ -> right(n - 1) } }
fn right(n) { case n { 0 -> 41 _ -> left(n - 1) } }
pub fn main() { left(20) + 1 }
"#,
                2,
            ),
        ] {
            let plan = crate::runtime::plan_src(source);
            let graph = plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .as_view();
            let root_instructions = graph.block(graph.entry()).instructions().as_ptr();
            let mut execution = Execution::new(graph, RetainedValues::empty());
            let mut returns = Returns::new();
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            let mut owner = None;
            let mut bodies = BTreeSet::new();
            let mut observations = 0;
            let completed = loop {
                if let Activation::Graph(frame) = &execution.active {
                    let instructions = frame
                        .graph
                        .block(frame.graph.entry())
                        .instructions()
                        .as_ptr();
                    if instructions != root_instructions {
                        let address = ptr::from_ref(frame.exit.as_ref()).cast::<()>();
                        assert_eq!(address, *owner.get_or_insert(address));
                        assert_eq!(returns.ints.len(), 1);
                        bodies.insert(instructions as usize);
                        observations += 1;
                    }
                }
                match execution
                    .advance(&plan, &mut state, &mut returns, &mut 0)
                    .unwrap()
                {
                    Progress::Continue(next) => execution = next,
                    Progress::Complete(completed) => break completed,
                    Progress::Host(never) => match never {},
                }
            };
            assert_eq!(returned_int(&plan, IntFunctionId(0), completed), 42.into());
            assert_eq!(bodies.len(), expected_bodies);
            assert!(observations > 20);
            assert!(returns.ints.is_empty());
            assert!(echo.is_empty());
        }
    }

    struct MapperLease(Arc<AtomicUsize>);

    impl Drop for MapperLease {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn tail_exit_keeps_its_once_mapper_until_return_or_abandonment() {
        use crate::plan::execution::graph::IntLocalId;

        let plan = crate::runtime::plan_src(
            r#"
fn walk(n) { case n { 0 -> 41 _ -> walk(n - 1) } }
pub fn main() { walk(100) + 1 }
"#,
        );
        let caller = plan
            .int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .as_view();
        for complete in [false, true] {
            let calls = Arc::new(AtomicUsize::new(0));
            let drops = Arc::new(AtomicUsize::new(0));
            let lease = MapperLease(drops.clone());
            let observed_calls = calls.clone();
            let mut returns = Returns::new();
            let destination = returns.suspend(Frame {
                graph: caller,
                position: GraphPosition::new(caller.entry(), RetainedValues::empty()),
                exit: Box::new(RootExit),
            });
            let mut inputs = RetainedValues::empty();
            inputs.push_evaluated(EvaluatedValue::Int(100.into()));
            let mut execution = Execution {
                active: enter_function(
                    &plan,
                    IntFunctionId(1),
                    HostCallOrigin::Entry,
                    inputs,
                    destination,
                    move |value: BigInt| {
                        observed_calls.fetch_add(1, Ordering::SeqCst);
                        drop(lease);
                        Ok(value + 1)
                    },
                ),
            };
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            for _ in 0..30 {
                execution = continuing(
                    execution
                        .advance(&plan, &mut state, &mut returns, &mut 0)
                        .unwrap(),
                );
            }
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            assert_eq!(drops.load(Ordering::SeqCst), 0);
            if complete {
                while !returns.ints.is_empty() {
                    execution = continuing(
                        execution
                            .advance(&plan, &mut state, &mut returns, &mut 0)
                            .unwrap(),
                    );
                }
                assert_eq!(
                    active_frame(&execution)
                        .position
                        .environment
                        .int(IntLocalId(0)),
                    42.into()
                );
                assert_eq!(calls.load(Ordering::SeqCst), 1);
                assert_eq!(drops.load(Ordering::SeqCst), 1);
            }
            drop(execution);
            drop(returns);
            assert_eq!(calls.load(Ordering::SeqCst), usize::from(complete));
            assert_eq!(drops.load(Ordering::SeqCst), 1);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn never_tail_calls_reuse_the_exit_owner_until_the_original_panic() {
        use crate::plan::execution::function::NeverFunctionId;

        let plan = crate::runtime::plan_src(
            r#"
fn left(n) { case n { 0 -> panic as "tail stopped" _ -> right(n - 1) } }
fn right(n) { left(n) }
pub fn main() { left(20) }
"#,
        );
        let mut execution = Execution {
            active: enter_never(
                &plan,
                NeverFunctionId(0),
                HostCallOrigin::Entry,
                RetainedValues::empty(),
            ),
        };
        let mut owner = None;
        let mut observations = 0;
        let mut returns = Returns::new();
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let error = loop {
            let address = ptr::from_ref(active_frame(&execution).exit.as_ref()).cast::<()>();
            assert_eq!(address, *owner.get_or_insert(address));
            observations += 1;
            match execution.advance(&plan, &mut state, &mut returns, &mut 0) {
                Ok(progress) => execution = continuing(progress),
                Err(error) => break error,
            }
        };
        assert!(observations > 20);
        assert_eq!(error.to_string(), "panic: tail stopped");
        assert!(echo.is_empty());
    }

    #[test]
    fn nested_calls_transfer_between_steps_without_replaying_echo_or_losing_captures() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Boxed { Boxed(List(Int)) }

fn nested(value: Int) -> Int {
  echo value
  case value {
    0 -> 0
    _ -> nested(value - 1) + 1
  }
}

pub fn main() {
  let offset = 40
  let add = fn(value) { offset + nested(value) }
  let boxed = Boxed([1, 2])
  let Boxed(values) = boxed
  echo values
  case values {
    [first, ..] -> add(first) + 1
    [] -> 0
  }
}
"#,
        );
        let body = plan.int_function(IntFunctionId(0)).body();
        let mut execution = Execution::new(body.block_graph().as_view(), RetainedValues::empty());
        let mut returns = Returns::new();
        let mut lists = RuntimeListStorage::default();
        let captures = crate::runtime::CaptureStorage::default();
        let mut output = Vec::new();
        let mut steps = 0;
        let (completed, output) = std::thread::scope(|scope| {
            loop {
                let plan = &plan;
                let captures = captures.clone();
                let (step, retained_returns, retained_lists, retained_output) = scope
                    .spawn(move || {
                        let mut state =
                            RuntimeState::with_host_storage(&mut output, (), lists, captures);
                        let step = execution
                            .advance(plan, &mut state, &mut returns, &mut 0)
                            .expect("one actual evaluator step");
                        let lists = state.lists().clone();
                        drop(state);
                        (step, returns, lists, output)
                    })
                    .join()
                    .expect("owned activation transfers");
                steps += 1;
                assert!(steps < 200, "the finite source must make progress");
                lists = retained_lists;
                returns = retained_returns;
                output = retained_output;
                match step {
                    Progress::Continue(next) => execution = next,
                    Progress::Host(invoke) => match invoke {},
                    Progress::Complete(completed) => break (completed, output),
                }
            }
        });
        assert_eq!(returned_int(&plan, IntFunctionId(0), completed), 42.into());
        assert!(steps > 20);
        assert_eq!(
            output
                .iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["[1, 2]", "1", "0"]
        );
    }

    #[test]
    fn deep_non_tail_calls_use_owned_frames_instead_of_the_rust_stack() {
        let plan = crate::runtime::plan_src(
            r#"
fn count(value: Int) -> Int {
  case value {
    0 -> 0
    _ -> count(value - 1) + 1
  }
}
pub fn main() { count(20_000) }
"#,
        );
        let result = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || crate::run_main(&plan, &mut Vec::new()))
            .expect("small stack worker")
            .join()
            .expect("non-tail execution completes");
        assert_eq!(result, Ok(Value::Int(20_000.into())));
    }

    #[test]
    fn abandoning_a_deep_call_chain_drops_frames_without_recursive_destruction() {
        let plan = crate::runtime::plan_src(
            r#"
fn count(value: Int) -> Int { count(value + 1) + 1 }
pub fn main() { count(0) + 1 }
"#,
        );
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let body = plan.int_function(IntFunctionId(0)).body();
                let mut execution =
                    Execution::new(body.block_graph().as_view(), RetainedValues::empty());
                let mut returns = Returns::new();
                let mut echo = Vec::new();
                let mut state = RuntimeState::new(&mut echo);
                for _ in 0..50_000 {
                    execution = continuing(
                        execution
                            .advance(&plan, &mut state, &mut returns, &mut 0)
                            .expect("recursive source step"),
                    );
                }
                assert!(returns.ints.len() > 5_000);
                drop(execution);
                drop(returns);
            })
            .expect("small stack worker")
            .join()
            .expect("flat activation destruction");
    }

    #[test]
    fn source_and_native_never_entries_propagate_their_original_failure() {
        use crate::{HostFailure, HostProviderModule, HostProviderSet, StatelessHostProfile};
        use std::convert::Infallible;

        for (body, expected) in [
            ("let stop = source_stop stop()", "panic: source stopped"),
            ("source_stop()", "panic: source stopped"),
            (
                "let stop = native_stop stop()",
                "host function application::main.native_stop failed: native stopped",
            ),
            (
                "native_stop()",
                "host function application::main.native_stop failed: native stopped",
            ),
        ] {
            let source = format!(
                r#"
@external(erlang, "native", "stop")
fn native_stop() -> value
fn source_stop() {{ panic as "source stopped" }}
fn forward() {{ {body} }}
pub fn main() {{ let stop = forward stop() }}
"#
            );
            let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
                .unwrap()
                .with_fallible_function("native_stop", || -> Result<Infallible, HostFailure> {
                    Err(HostFailure::new("native stopped"))
                })
                .unwrap();
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [crate::PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "main",
                        "main.gleam",
                        source.as_str(),
                    )],
                )],
                HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let host = crate::execution_fixture::TestHost::default();
            let mut echo = Vec::new();
            let error = host
                .block_on(execution.run_main(&host, &mut (), &mut echo))
                .unwrap_err();
            assert_eq!(error.to_string(), expected);
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn graph_driver_propagates_instruction_and_return_invariants_with_prior_echo() {
        use crate::execution_fixture::TestHost;
        use crate::plan::execution::function::{FunctionReturnFamily, TupleFunctionId};
        use crate::runtime::execution::Domain;
        use crate::runtime::{ExecutionError, InvariantError};
        use crate::{
            HostProviderSet, HostedExecution, ModuleSource, PackageSource, StatelessHostProfile,
        };
        let source = r#"
fn project(value: #(Int)) { echo "projection" value.0 + 1 }
fn apply(factory: fn() -> fn() -> Int) {
  echo "return"
  let callback = factory()
  callback()
}
fn factory() { fn() { 1.5 } }
pub fn main() { #(project, apply, factory) }
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = execution.parts_mut();
        let host = TestHost::default();
        let mut state = ();
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<StatelessHostProfile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let values = context
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 3);
            for (entry, input, expected) in [
                (
                    IntFunctionId(0),
                    EvaluatedValue::Tuple(vec![EvaluatedValue::Bool(true)]),
                    InvariantError::TupleIndexFamilyMismatch {
                        expected: crate::ValueType::Int,
                        actual: crate::ValueType::Bool,
                    },
                ),
                (
                    IntFunctionId(1),
                    values[2].clone(),
                    InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::Int,
                        actual: FunctionReturnFamily::Float,
                    },
                ),
            ] {
                let mut inputs = RetainedValues::empty();
                inputs.push_evaluated(input);
                assert_eq!(
                    context
                        .call(entry, HostCallOrigin::Entry, inputs)
                        .await
                        .unwrap(),
                    Err(ExecutionError::Invariant(expected))
                );
            }
        }))
        .unwrap();
        assert_eq!(
            echo.iter()
                .map(|output| output.value().clone())
                .collect::<Vec<_>>(),
            vec![
                Value::String("projection".into()),
                Value::String("return".into())
            ]
        );
    }

    #[test]
    fn source_and_native_function_returns_check_the_evaluated_target_family() {
        use crate::execution_fixture::TestHost;
        use crate::host::{
            HostCall, HostCallCompletion, HostCallError, HostCallable, HostFunctionType,
            HostProfile, HostProvider, HostProviderModule, HostProviderSet, HostTypeListEnd,
        };
        use crate::plan::execution::function::{FunctionReturnFamily, TupleFunctionId};
        use crate::runtime::execution::Domain;
        use crate::runtime::{ExecutionError, HostCallOrigin, InvariantError};
        use std::sync::Arc;

        struct Profile;
        impl HostProfile for Profile {
            type RunState = usize;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        struct Provider;
        impl HostProvider<Profile> for Provider {
            type State = usize;
            fn project(state: &mut usize) -> &mut usize {
                state
            }
        }
        type Callback = HostFunctionType<HostTypeListEnd, f64>;
        fn forward<'call>(
            mut call: HostCall<'call, Profile, Provider, Callback>,
            callback: HostCallable<'call, HostTypeListEnd, f64>,
        ) -> Result<HostCallCompletion<'call, Callback>, HostCallError> {
            *call.state() += 1;
            Ok(call.return_value(callback))
        }

        let source = r#"
@external(erlang, "native", "forward")
fn forward(value: fn() -> Float) -> fn() -> Float

fn apply_int(factory: fn(fn() -> Float) -> fn() -> Int, value: fn() -> Float) {
  let result = factory(value)
  result()
}
fn apply_float(factory: fn(fn() -> Float) -> fn() -> Float, value: fn() -> Float) {
  let result = factory(value)
  let assert 1.5 = result()
  42
}
fn integer(_value: fn() -> Float) { fn() { 42 } }
fn floating(value: fn() -> Float) { leaf(value) }
fn leaf(value: fn() -> Float) { value }
fn native_tail(value: fn() -> Float) { forward(value) }
pub fn main() { #(apply_int, apply_float, integer, floating, forward, fn() { 1.5 }, native_tail) }
"#;
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [crate::PackageSource::new(
                "application",
                Vec::<String>::new(),
                [crate::ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::from_providers([HostProviderModule::new("application", "main")
                .unwrap()
                .with_scoped_function::<Provider, (Callback,), Callback, _>("forward", forward)
                .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, stores, captures) = execution.parts_mut();
        let host = TestHost::default();
        let mut state = 0;
        let mut echo = Vec::new();
        let domain = Domain::new(
            Arc::clone(plan),
            &host,
            &mut state,
            stores,
            &mut echo,
            captures.clone(),
            Domain::<Profile>::DEFAULT_BUDGET,
        );
        let context = domain.context();
        host.block_on(domain.drive(async {
            let values = context
                .call(
                    TupleFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .await
                .unwrap()
                .unwrap();
            assert_eq!(values.len(), 7);
            for (entry, factory, expected) in [
                (IntFunctionId(0), 2, Ok(42.into())),
                (IntFunctionId(1), 3, Ok(42.into())),
                (IntFunctionId(1), 4, Ok(42.into())),
                (IntFunctionId(1), 6, Ok(42.into())),
                (
                    IntFunctionId(0),
                    3,
                    Err(ExecutionError::Invariant(
                        InvariantError::FunctionReturnFamilyMismatch {
                            expected: FunctionReturnFamily::Int,
                            actual: FunctionReturnFamily::Float,
                        },
                    )),
                ),
                (
                    IntFunctionId(0),
                    4,
                    Err(ExecutionError::Invariant(
                        InvariantError::FunctionReturnFamilyMismatch {
                            expected: FunctionReturnFamily::Int,
                            actual: FunctionReturnFamily::Float,
                        },
                    )),
                ),
                (
                    IntFunctionId(0),
                    6,
                    Err(ExecutionError::Invariant(
                        InvariantError::FunctionReturnFamilyMismatch {
                            expected: FunctionReturnFamily::Int,
                            actual: FunctionReturnFamily::Float,
                        },
                    )),
                ),
            ] {
                // The last three cases corrupt only the evaluated factory argument.
                // The compiled entries, native adapter and return slots are unchanged.
                let mut inputs = RetainedValues::empty();
                inputs.push_evaluated(values[factory].clone());
                inputs.push_evaluated(values[5].clone());
                let result = context
                    .call(entry, HostCallOrigin::Entry, inputs)
                    .await
                    .unwrap();
                assert_eq!(result, expected);
            }
        }))
        .unwrap();
        assert_eq!(state, 4);
        assert!(echo.is_empty());
    }
}
