use super::super::{EvaluatedFunctionExit, evaluate_entry, parameter_locals};
use crate::plan::execution::function::BoolFunctionId;
use crate::plan::execution::graph::ParamLocal;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_bool<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: BoolFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<bool> {
    loop {
        let exit = evaluate_entry(plan, state, plan.bool_function(function), origin, inputs)?;
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

pub(in crate::runtime) fn bool_parameter_locals<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    function: BoolFunctionId,
) -> Vec<ParamLocal> {
    parameter_locals(plan, plan.bool_function(function))
}

#[cfg(test)]
mod tests {
    use super::bool_parameter_locals;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId, ParamLocal};
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
        assert_eq!(
            bool_parameter_locals(&execution, BoolFunctionId(1)),
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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        assert_eq!(
            bool_parameter_locals(&execution, BoolFunctionId(2)),
            [ParamLocal::Bool(BoolLocalId(0))],
        );
        assert_eq!(
            bool_parameter_locals(&execution, BoolFunctionId(1)),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Bool(true)),
        );
    }
}
