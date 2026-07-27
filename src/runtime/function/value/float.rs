use super::super::{EvaluatedFunctionExit, evaluate};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, FloatFunctionId,
};
use crate::plan::execution::graph::ParamLocal;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_float<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: FloatFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<f64> {
    loop {
        let exit = match plan.float_function(function).as_ref() {
            ExecutionFunctionRef::Graph(function) => evaluate(plan, state, function.body(), inputs),
            ExecutionFunctionRef::Host(target) => plan
                .call_host_float(state, origin.clone(), target, &inputs)
                .map(EvaluatedFunctionExit::Return),
        }?;
        match exit {
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

pub(in crate::runtime) fn float_parameter_locals<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    function: FloatFunctionId,
) -> Vec<ParamLocal> {
    match plan.float_function(function).as_ref() {
        ExecutionFunctionRef::Graph(function) => function
            .entry()
            .params(function.body())
            .iter()
            .map(|slot| slot.local().clone())
            .collect(),
        ExecutionFunctionRef::Host(target) => plan.host_float_parameters(target).to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::float_parameter_locals;
    use crate::plan::execution::function::FloatFunctionId;
    use crate::plan::execution::graph::{FloatLocalId, ParamLocal};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, compile_typed_module, plan_host_program, plan_module, run_main,
    };

    #[test]
    fn plain_float_function_protocol_executes_graph_entries() {
        let source = r#"
fn identity(value: Float) {
  value
}

pub fn main() {
  identity(1.5)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            float_parameter_locals(&execution, FloatFunctionId(1)),
            [ParamLocal::Float(FloatLocalId(0))],
        );
        assert_eq!(run_main(&execution, &mut Vec::new()), Ok(Value::Float(1.5)),);
    }

    #[test]
    fn hosted_float_function_protocol_executes_graph_and_host_entries() {
        let numbers = HostModule::new("host_support", "host/numbers")
            .expect("host module should be valid")
            .with_function("identity", |value: f64| value)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([numbers]).expect("host modules should be unique");
        let source = r#"
import host/numbers

fn identity(value: Float) {
  value
}

pub fn main() {
  identity(numbers.identity(1.5))
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
        assert_eq!(
            float_parameter_locals(&execution, FloatFunctionId(2)),
            [ParamLocal::Float(FloatLocalId(0))],
        );
        assert_eq!(
            float_parameter_locals(&execution, FloatFunctionId(1)),
            [ParamLocal::Float(FloatLocalId(0))],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Float(1.5)),
        );
    }
}
