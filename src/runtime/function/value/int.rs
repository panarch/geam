use super::super::{EvaluatedFunctionExit, evaluate_entry, parameter_locals};
use crate::plan::execution::function::IntFunctionId;
use crate::plan::execution::graph::ParamLocal;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;
use num_bigint::BigInt;

pub(in crate::runtime) fn run_int<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: IntFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<BigInt> {
    loop {
        let exit = evaluate_entry(plan, state, plan.int_function(function), origin, inputs)?;
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

pub(in crate::runtime) fn int_parameter_locals<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    function: IntFunctionId,
) -> Vec<ParamLocal> {
    parameter_locals(plan, plan.int_function(function))
}

#[cfg(test)]
mod tests {
    use super::int_parameter_locals;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{IntLocalId, ParamLocal};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
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
        assert_eq!(
            int_parameter_locals(&execution, IntFunctionId(1)),
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
        let hosts = HostProviderSet::new([math]).expect("host modules should be unique");
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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        assert_eq!(
            int_parameter_locals(&execution, IntFunctionId(2)),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            int_parameter_locals(&execution, IntFunctionId(1)),
            [
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::Int(42.into())),
        );
    }
}
