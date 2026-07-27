use super::{evaluate, run_tail};
use crate::plan::execution::HostedExecution;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExecutableFunction, FloatFunctionId,
    IntFunctionBody, IntFunctionEntry, IntFunctionId, NeverFunctionId, NilFunctionId,
    StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::graph::{IntLocalId, ParamLocal};
use crate::plan::execution::host::HostIntFunctionId;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedCustomValue, EvaluatedNeverFunction, EvaluatedValue,
};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use ecow::EcoString;
use num_bigint::BigInt;
use std::convert::Infallible;

pub(in crate::runtime) trait RuntimeIntFunction<Plan: RuntimeExecutionPlan> {
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &Plan,
        state: &mut RuntimeState,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<super::EvaluatedFunctionExit<BigInt, IntFunctionId>>
    where
        EvaluateGraph: FnOnce(
            &Plan,
            &mut RuntimeState,
            &ExecutableFunction<IntFunctionBody>,
            RetainedValues,
        ) -> ExecutionResult<
            super::EvaluatedFunctionExit<BigInt, IntFunctionId>,
        >;

    fn parameter_locals(&self, plan: &Plan) -> Vec<ParamLocal>;
}

impl<Plan: RuntimeExecutionPlan> RuntimeIntFunction<Plan> for ExecutableFunction<IntFunctionBody> {
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &Plan,
        state: &mut RuntimeState,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<super::EvaluatedFunctionExit<BigInt, IntFunctionId>>
    where
        EvaluateGraph: FnOnce(
            &Plan,
            &mut RuntimeState,
            &ExecutableFunction<IntFunctionBody>,
            RetainedValues,
        ) -> ExecutionResult<
            super::EvaluatedFunctionExit<BigInt, IntFunctionId>,
        >,
    {
        evaluate_graph(plan, state, self, inputs)
    }

    fn parameter_locals(&self, _plan: &Plan) -> Vec<ParamLocal> {
        self.entry()
            .params(self.body())
            .iter()
            .map(|slot| slot.local().clone())
            .collect()
    }
}

impl RuntimeIntFunction<HostedExecution> for IntFunctionEntry<HostIntFunctionId> {
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &HostedExecution,
        state: &mut RuntimeState,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<super::EvaluatedFunctionExit<BigInt, IntFunctionId>>
    where
        EvaluateGraph: FnOnce(
            &HostedExecution,
            &mut RuntimeState,
            &ExecutableFunction<IntFunctionBody>,
            RetainedValues,
        ) -> ExecutionResult<
            super::EvaluatedFunctionExit<BigInt, IntFunctionId>,
        >,
    {
        match self {
            IntFunctionEntry::Graph(function) => evaluate_graph(plan, state, function, inputs),
            IntFunctionEntry::Host(target) => Ok(super::EvaluatedFunctionExit::Return(
                plan.host_int_function(*target)
                    .call(inputs.int_argument(0), inputs.int_argument(1)),
            )),
        }
    }

    fn parameter_locals(&self, plan: &HostedExecution) -> Vec<ParamLocal> {
        match self {
            IntFunctionEntry::Graph(function) => function.parameter_locals(plan),
            IntFunctionEntry::Host(_) => vec![
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Int(IntLocalId(1)),
            ],
        }
    }
}

pub(in crate::runtime) fn run_never(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: NeverFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.never_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_int(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    mut function: IntFunctionId,
    mut inputs: RetainedValues,
) -> ExecutionResult<BigInt> {
    loop {
        match plan.int_function(function).evaluate(
            plan,
            state,
            inputs,
            |plan, state, function, inputs| evaluate(plan, state, function.body(), inputs),
        )? {
            super::EvaluatedFunctionExit::Return(value) => return Ok(value),
            super::EvaluatedFunctionExit::TailCall {
                function: target,
                args,
            } => {
                function = target;
                inputs = args;
            }
        }
    }
}

pub(in crate::runtime) fn run_float(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: FloatFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<f64> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.float_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_string(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: StringFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EcoString> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.string_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_bit_array(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: BitArrayFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArray> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.bit_array_function(*function).body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_utf_codepoint(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: UtfCodepointFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<char> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.utf_codepoint_function(*function).body(),
                inputs,
            )
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_custom(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: CustomFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomValue> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(
                plan,
                state,
                plan.custom_function(*function).body().function_body(),
                inputs,
            )
        },
        |plan, function, target| plan.custom_function(*function).body().function_id(target),
    )
}

pub(in crate::runtime) fn run_bool(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: BoolFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<bool> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.bool_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_nil(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: NilFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<()> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.nil_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_tuple(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: TupleFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<Vec<EvaluatedValue>> {
    run_tail(
        plan,
        state,
        function,
        inputs,
        |plan, state, function, inputs| {
            evaluate(plan, state, plan.tuple_function(*function).body(), inputs)
        },
        |_, _, target| target,
    )
}

pub(in crate::runtime) fn run_never_value(
    plan: &impl ExecutableRuntimePlan,
    state: &mut RuntimeState,
    function: EvaluatedNeverFunction,
    mut inputs: RetainedValues,
) -> ExecutionResult<Infallible> {
    inputs.append_captures(function.captures());
    run_never(plan, state, function.runtime_id(), inputs)
}

#[cfg(test)]
mod tests {
    use super::RuntimeIntFunction as _;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{IntLocalId, ParamLocal};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::{
        HostModule, HostModules, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, compile_typed_module, plan_host_program, plan_module, run_main,
    };
    use num_bigint::BigInt;

    #[test]
    fn plain_int_function_protocol_executes_graph_entries() {
        let source = r#"
fn increment(value: Int) {
  value + 1
}

pub fn main() {
  increment(41)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        let function = RuntimeExecutionPlan::int_function(&execution, IntFunctionId(1));

        assert_eq!(
            function.parameter_locals(&execution),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            run_main(&execution, &mut Vec::new()),
            Ok(Value::Int(42.into()))
        );
    }

    #[test]
    fn hosted_int_function_protocol_executes_graph_and_host_entries() {
        let math = HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid");
        let hosts = HostModules::new([math]).expect("host modules should be unique");
        let source = r#"
import host/math

fn increment(value: Int) {
  value + 1
}

pub fn main() {
  increment(math.add(20, 21))
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            hosts,
        )
        .expect("host source should compile");
        let plan = plan_host_program(typed).expect("host source should plan");
        let execution = HostedExecution::from_module_plan(plan);
        let host = RuntimeExecutionPlan::int_function(&execution, IntFunctionId(1));
        let graph = RuntimeExecutionPlan::int_function(&execution, IntFunctionId(2));

        assert_eq!(
            graph.parameter_locals(&execution),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            host.parameter_locals(&execution),
            [
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(
            execution.run_main(&mut Vec::new()),
            Ok(Value::Int(42.into())),
        );
    }
}
