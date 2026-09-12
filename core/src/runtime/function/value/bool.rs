use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::BoolFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_bool(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BoolFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<bool> {
    run(plan, state, function, origin, inputs)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::FunctionTarget;
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
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::Bool(BoolFunctionId(1))),
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
        let mut execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        assert_eq!(
            execution
                .execution()
                .function_parameters()
                .function(&FunctionTarget::Bool(BoolFunctionId(2))),
            [ParamLocal::Bool(BoolLocalId(0))],
        );
        assert_eq!(
            execution
                .execution()
                .function_parameters()
                .function(&FunctionTarget::Bool(BoolFunctionId(1))),
            [ParamLocal::Int(IntLocalId(0))],
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
            Ok(Value::Bool(true)),
        );
    }
}
