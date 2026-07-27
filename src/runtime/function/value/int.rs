use super::super::{EvaluatedFunctionExit, evaluate};
use crate::plan::execution::HostedExecution;
use crate::plan::execution::function::{
    ExecutableFunction, IntFunctionBody, IntFunctionId, ValueFunctionEntry,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::host::HostIntFunctionId;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::ExecutionResult;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use num_bigint::BigInt;

pub(in crate::runtime) trait RuntimeIntFunction<Plan: RuntimeExecutionPlan> {
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &Plan,
        state: &mut RuntimeState,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<EvaluatedFunctionExit<BigInt, IntFunctionId>>
    where
        EvaluateGraph: FnOnce(
            &Plan,
            &mut RuntimeState,
            &ExecutableFunction<IntFunctionBody>,
            RetainedValues,
        )
            -> ExecutionResult<EvaluatedFunctionExit<BigInt, IntFunctionId>>;

    fn parameter_locals(&self, plan: &Plan) -> Vec<ParamLocal>;
}

impl<Plan: RuntimeExecutionPlan> RuntimeIntFunction<Plan> for ExecutableFunction<IntFunctionBody> {
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &Plan,
        state: &mut RuntimeState,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<EvaluatedFunctionExit<BigInt, IntFunctionId>>
    where
        EvaluateGraph: FnOnce(
            &Plan,
            &mut RuntimeState,
            &ExecutableFunction<IntFunctionBody>,
            RetainedValues,
        )
            -> ExecutionResult<EvaluatedFunctionExit<BigInt, IntFunctionId>>,
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

impl RuntimeIntFunction<HostedExecution>
    for ValueFunctionEntry<IntFunctionBody, HostIntFunctionId>
{
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &HostedExecution,
        state: &mut RuntimeState,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<EvaluatedFunctionExit<BigInt, IntFunctionId>>
    where
        EvaluateGraph: FnOnce(
            &HostedExecution,
            &mut RuntimeState,
            &ExecutableFunction<IntFunctionBody>,
            RetainedValues,
        )
            -> ExecutionResult<EvaluatedFunctionExit<BigInt, IntFunctionId>>,
    {
        match self {
            ValueFunctionEntry::Graph(function) => evaluate_graph(plan, state, function, inputs),
            ValueFunctionEntry::Host(target) => Ok(EvaluatedFunctionExit::Return(
                plan.host_int_function(*target).call(&inputs),
            )),
        }
    }

    fn parameter_locals(&self, plan: &HostedExecution) -> Vec<ParamLocal> {
        match self {
            ValueFunctionEntry::Graph(function) => function.parameter_locals(plan),
            ValueFunctionEntry::Host(target) => {
                plan.host_int_function(*target).parameters().to_vec()
            }
        }
    }
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
            EvaluatedFunctionExit::Return(value) => return Ok(value),
            EvaluatedFunctionExit::TailCall {
                function: target,
                args,
            } => {
                function = target;
                inputs = args;
            }
        }
    }
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
