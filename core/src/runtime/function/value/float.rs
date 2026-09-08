use super::super::{EvaluatedFunctionExit, evaluate_entry};
use crate::plan::execution::function::FloatFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::ProfiledRetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_float<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: FloatFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<f64, Plan::Values> {
    loop {
        let exit = evaluate_entry(plan, state, plan.float_function(function), origin, inputs)?;
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::FloatFunctionId;
    use crate::plan::execution::graph::FunctionTarget;
    use crate::plan::execution::graph::{FloatLocalId, ParamLocal};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
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
            execution
                .function_parameters()
                .function(&FunctionTarget::Float(FloatFunctionId(1))),
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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::Float(FloatFunctionId(2))),
            [ParamLocal::Float(FloatLocalId(0))],
        );
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::Float(FloatFunctionId(1))),
            [ParamLocal::Float(FloatLocalId(0))],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Float(1.5)),
        );
    }
}
