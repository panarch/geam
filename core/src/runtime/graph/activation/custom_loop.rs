use super::{Activation, Frame, GraphExit, GraphPosition, ReturnValue, Storage};
use crate::plan::execution::compiled::{
    CompiledCallbackBody, CompiledCheckpoint, CompiledLoopFunction, CustomLoopImplementation,
};
use crate::plan::execution::function::{ExecutionBoolFunctionBody, ExecutionIntFunctionBody};
use crate::plan::execution::graph::{BoolLocalId, IntLocalId, ParamLocal};
use crate::runtime::captures::Captures;
use crate::runtime::compiled::CompiledProgress;
use crate::runtime::compiled::custom_loop::{
    CallbackStop, CustomListOps, CustomLoopProgress, CustomLoopValues,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::{CompletedGraph, GraphValue, RetainedValues, RuntimeGraphState};
use crate::runtime::{ExecutableRuntimePlan, ExecutionError};

/// Only a selected connection owns this state. General frames and typed return
/// stacks retain their existing layout and behavior.
pub(in crate::runtime::graph) struct CustomLoopExecution<'plan, Plan: ExecutableRuntimePlan> {
    frame: Frame<'plan, Plan>,
    implementation: &'plan CustomLoopImplementation,
    point: usize,
    values: CustomLoopValues,
    loaded: bool,
    ints:
        &'plan [CompiledCallbackBody<'plan, ExecutionIntFunctionBody<Plan::Profile>, IntLocalId>],
    bools: &'plan [CompiledCallbackBody<
        'plan,
        ExecutionBoolFunctionBody<Plan::Profile>,
        BoolLocalId,
    >],
}

impl<'plan, Plan: ExecutableRuntimePlan> CustomLoopExecution<'plan, Plan> {
    pub(super) fn enter(
        frame: Frame<'plan, Plan>,
        implementation: &'plan CustomLoopImplementation,
        point: usize,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = ExecutionError>,
        storage: &mut Storage<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        let callbacks = plan.compiled_callbacks();
        if !frame
            .position
            .environment
            .supports_custom_loop(callbacks, state.captures().domain())
        {
            return frame.advance(plan, state, storage, remaining);
        }
        let bodies = plan.compiled_callback_bodies();
        Box::new(Self {
            frame,
            implementation,
            point,
            values: CustomLoopValues::default(),
            loaded: false,
            ints: &bodies.ints,
            bools: &bodies.bools,
        })
        .advance(plan, state, storage, remaining)
    }

    pub(super) fn advance(
        mut self: Box<Self>,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = ExecutionError>,
        storage: &mut Storage<'plan, Plan>,
        remaining: &mut usize,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        if !self.loaded {
            self.loaded = self.frame.position.environment.load_custom_loop(
                &mut self.values,
                plan.compiled_callbacks(),
                state.captures().domain(),
            );
            if !self.loaded {
                return self.interpreted_step(plan, state, storage);
            }
        }
        let mut budget = *remaining + 1;
        let progress = (self.implementation.run)(
            self.point,
            &mut self.values,
            &CustomListOps::new(state.lists()),
            &mut budget,
        );
        *remaining = budget;
        match progress {
            CustomLoopProgress::Caller(CompiledProgress::Yield(point)) => {
                self.point = point;
                Ok(Activation::CustomLoop(self))
            }
            CustomLoopProgress::Caller(CompiledProgress::Interpreted(point)) => {
                self.point = point;
                self.restore();
                if *remaining == 0 {
                    Ok(Activation::CustomLoop(self))
                } else {
                    *remaining -= 1;
                    self.interpreted_step(plan, state, storage)
                }
            }
            CustomLoopProgress::Caller(CompiledProgress::Complete(exit)) => {
                self.restore();
                self.frame.exit.exit(
                    CompletedGraph {
                        exit,
                        environment: self.frame.position.environment,
                        direct_return: false,
                    },
                    storage,
                )
            }
            CustomLoopProgress::Call { point, progress } => self.callback(point, progress, storage),
        }
    }

    fn restore(&mut self) {
        if self.loaded {
            self.frame
                .position
                .environment
                .restore_custom_loop(&mut self.values);
            self.loaded = false;
        }
        let point = self.implementation.checkpoints[self.point];
        self.frame.position.block = point.block;
        self.frame.position.instruction = point.instruction;
    }

    fn interpreted_step(
        mut self: Box<Self>,
        plan: &'plan Plan,
        state: &mut impl RuntimeGraphState<Error = ExecutionError>,
        storage: &mut Storage<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        if self
            .implementation
            .calls
            .iter()
            .any(|call| call.point == self.point)
        {
            let point = self.point;
            // A caller overflow may have restored and drained its bindings
            // immediately before this Call. Bind the actual canonical values
            // again before creating the cold callee; the Call is charged once.
            self.loaded = self.frame.position.environment.load_custom_loop(
                &mut self.values,
                plan.compiled_callbacks(),
                state.captures().domain(),
            );
            return self.callback(point, CallbackStop::Entry, storage);
        }
        let active = self.frame.advance(plan, state, storage, &mut 0)?;
        match active {
            Activation::Graph(frame) => {
                self.frame = frame;
                if let Some(point) = self.implementation.checkpoints.iter().position(|point| {
                    point.block == self.frame.position.block
                        && point.instruction == self.frame.position.instruction
                }) {
                    self.point = point;
                    Ok(Activation::CustomLoop(self))
                } else {
                    Ok(Activation::Graph(self.frame))
                }
            }
            active => Ok(active),
        }
    }

    fn callback(
        mut self: Box<Self>,
        point: usize,
        progress: CallbackStop,
        storage: &mut Storage<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        self.point = point;
        let call = &self.implementation.calls[self
            .implementation
            .calls
            .partition_point(|call| call.point < point)];
        let checkpoint = match progress {
            CallbackStop::Yield(point) | CallbackStop::Interpreted(point) => Some(point),
            CallbackStop::Entry => None,
        };
        match &call.function {
            CompiledLoopFunction::Int { local, .. } => {
                let function = &self.values.int_functions[local.0];
                let body = &self.ints[function.binding];
                let graph = body.body.block_graph().as_view();
                let returns = body.returns;
                let checkpoint = checkpoint.map(|point| body.checkpoints[point]);
                let captures = function.value.capture_frame().clone();
                self.restore();
                let inputs =
                    self.callback_inputs(checkpoint, call.args.as_ref(), &captures, storage);
                let mut position = GraphPosition::new(graph.entry(), inputs);
                if let Some(point) = checkpoint {
                    position.block = point.block;
                    position.instruction = point.instruction;
                }
                Ok(Activation::Graph(Frame {
                    graph,
                    position,
                    exit: Box::new(CallbackExit {
                        parent: self,
                        returns,
                    }),
                }))
            }
            CompiledLoopFunction::Bool { local, .. } => {
                let function = &self.values.bool_functions[local.0];
                let body = &self.bools[function.binding];
                let graph = body.body.block_graph().as_view();
                let returns = body.returns;
                let checkpoint = checkpoint.map(|point| body.checkpoints[point]);
                let captures = function.value.capture_frame().clone();
                self.restore();
                let inputs =
                    self.callback_inputs(checkpoint, call.args.as_ref(), &captures, storage);
                let mut position = GraphPosition::new(graph.entry(), inputs);
                if let Some(point) = checkpoint {
                    position.block = point.block;
                    position.instruction = point.instruction;
                }
                Ok(Activation::Graph(Frame {
                    graph,
                    position,
                    exit: Box::new(CallbackExit {
                        parent: self,
                        returns,
                    }),
                }))
            }
        }
    }

    fn callback_inputs(
        &mut self,
        checkpoint: Option<CompiledCheckpoint>,
        args: &[ParamLocal],
        captures: &Captures,
        storage: &mut Storage<'plan, Plan>,
    ) -> RetainedValues {
        let mut inputs = storage.pool.acquire();
        if checkpoint.is_some() {
            inputs.restore_custom(&mut self.values.callee);
        } else {
            inputs.append_locals(&self.frame.position.environment, args);
            inputs.append_captures(captures);
        }
        self.frame.position.instruction += 1;
        self.point += 1;
        inputs
    }
}

struct CallbackExit<'plan, Plan: ExecutableRuntimePlan, Local> {
    parent: Box<CustomLoopExecution<'plan, Plan>>,
    returns: &'plan [Local],
}

impl<'plan, Plan, Local> GraphExit<'plan, Plan> for CallbackExit<'plan, Plan, Local>
where
    Plan: ExecutableRuntimePlan,
    Local: GraphValue + Sync + 'plan,
    Local::Evaluated: ReturnValue,
{
    fn exit(
        self: Box<Self>,
        completed: CompletedGraph,
        storage: &mut Storage<'plan, Plan>,
    ) -> ExecutionResult<Activation<'plan, Plan>> {
        let local = &self.returns[completed.exit().0];
        let value = completed.into_value_and_recycle(local, &mut storage.pool);
        let mut parent = self.parent;
        value.push(&mut parent.frame.position.environment);
        Ok(Activation::CustomLoop(parent))
    }
}
