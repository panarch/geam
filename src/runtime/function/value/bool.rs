use super::super::{EvaluatedFunctionExit, evaluate};
use crate::plan::execution::HostedExecution;
use crate::plan::execution::function::{
    BoolFunctionBody, BoolFunctionId, ExecutableFunction, ValueFunctionEntry,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::host::HostBoolFunctionId;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) trait RuntimeBoolFunction<Plan: RuntimeExecutionPlan> {
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        origin: HostCallOrigin,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<EvaluatedFunctionExit<bool, crate::plan::FunctionCallTarget<BoolFunctionId>>>
    where
        EvaluateGraph: FnOnce(
            &Plan,
            &mut RuntimeStateFor<'_, Plan>,
            &ExecutableFunction<BoolFunctionBody>,
            RetainedValues,
        ) -> ExecutionResult<
            EvaluatedFunctionExit<bool, crate::plan::FunctionCallTarget<BoolFunctionId>>,
        >;

    fn parameter_locals(&self, plan: &Plan) -> Vec<ParamLocal>;
}

impl<Plan: RuntimeExecutionPlan> RuntimeBoolFunction<Plan>
    for ExecutableFunction<BoolFunctionBody>
{
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &Plan,
        state: &mut RuntimeStateFor<'_, Plan>,
        _origin: HostCallOrigin,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<EvaluatedFunctionExit<bool, crate::plan::FunctionCallTarget<BoolFunctionId>>>
    where
        EvaluateGraph: FnOnce(
            &Plan,
            &mut RuntimeStateFor<'_, Plan>,
            &ExecutableFunction<BoolFunctionBody>,
            RetainedValues,
        ) -> ExecutionResult<
            EvaluatedFunctionExit<bool, crate::plan::FunctionCallTarget<BoolFunctionId>>,
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

impl<Profile: crate::HostProfile> RuntimeBoolFunction<HostedExecution<Profile>>
    for ValueFunctionEntry<BoolFunctionBody, HostBoolFunctionId>
{
    fn evaluate<EvaluateGraph>(
        &self,
        plan: &HostedExecution<Profile>,
        state: &mut RuntimeStateFor<'_, HostedExecution<Profile>>,
        origin: HostCallOrigin,
        inputs: RetainedValues,
        evaluate_graph: EvaluateGraph,
    ) -> ExecutionResult<EvaluatedFunctionExit<bool, crate::plan::FunctionCallTarget<BoolFunctionId>>>
    where
        EvaluateGraph: FnOnce(
            &HostedExecution<Profile>,
            &mut RuntimeStateFor<'_, HostedExecution<Profile>>,
            &ExecutableFunction<BoolFunctionBody>,
            RetainedValues,
        ) -> ExecutionResult<
            EvaluatedFunctionExit<bool, crate::plan::FunctionCallTarget<BoolFunctionId>>,
        >,
    {
        match self {
            ValueFunctionEntry::Graph(function) => evaluate_graph(plan, state, function, inputs),
            ValueFunctionEntry::Host(target) => {
                let function = plan.host_bool_function(*target);
                function
                    .call(state.host_state(), &inputs)
                    .map(EvaluatedFunctionExit::Return)
                    .map_err(|error| {
                        let site = origin.into_site(function.site());
                        crate::runtime::ExecutionError::from_host_call(
                            function.metadata(),
                            site.clone(),
                            plan.source_context_for(site.module()),
                            error,
                        )
                    })
            }
        }
    }

    fn parameter_locals(&self, plan: &HostedExecution<Profile>) -> Vec<ParamLocal> {
        match self {
            ValueFunctionEntry::Graph(function) => function.parameter_locals(plan),
            ValueFunctionEntry::Host(target) => {
                plan.host_bool_function(*target).parameters().to_vec()
            }
        }
    }
}

pub(in crate::runtime) fn run_bool<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: BoolFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<bool> {
    loop {
        match plan.bool_function(function).evaluate(
            plan,
            state,
            origin,
            inputs,
            |plan, state, function, inputs| evaluate(plan, state, function.body(), inputs),
        )? {
            EvaluatedFunctionExit::Return(value) => return Ok(value),
            EvaluatedFunctionExit::TailCall {
                function: target,
                args,
            } => {
                origin = HostCallOrigin::source(target.site().clone());
                function = *target.function();
                inputs = args;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeBoolFunction as _;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId, ParamLocal};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, compile_typed_module, plan_host_program, plan_module, run_main,
    };
    use num_bigint::BigInt;

    #[test]
    fn plain_bool_function_protocol_executes_graph_entries() {
        let source = r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  identity(True)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        let function = RuntimeExecutionPlan::bool_function(&execution, BoolFunctionId(1));

        assert_eq!(
            function.parameter_locals(&execution),
            [ParamLocal::Bool(BoolLocalId(0))],
        );
        assert_eq!(run_main(&execution, &mut Vec::new()), Ok(Value::Bool(true)),);
    }

    #[test]
    fn hosted_bool_function_protocol_executes_graph_and_host_entries() {
        let predicates = HostModule::new("host_support", "host/predicates")
            .expect("host module should be valid")
            .with_function("is_positive", |value: BigInt| value > BigInt::from(0))
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([predicates]).expect("host modules should be unique");
        let source = r#"
import host/predicates

fn identity(value: Bool) {
  value
}

pub fn main() {
  identity(predicates.is_positive(1))
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
        let host = RuntimeExecutionPlan::bool_function(&execution, BoolFunctionId(1));
        let graph = RuntimeExecutionPlan::bool_function(&execution, BoolFunctionId(2));

        assert_eq!(
            graph.parameter_locals(&execution),
            [ParamLocal::Bool(BoolLocalId(0))],
        );
        assert_eq!(
            host.parameter_locals(&execution),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Bool(true)),
        );
    }
}
