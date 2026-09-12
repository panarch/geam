use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::StringFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use ecow::EcoString;

pub(in crate::runtime) fn run_string(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: StringFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EcoString> {
    run(plan, state, function, origin, inputs)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::StringFunctionId;
    use crate::plan::execution::graph::FunctionTarget;
    use crate::plan::execution::graph::{ParamLocal, StringLocalId};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, compile_typed_module, plan_host_program, plan_module, run_main,
    };
    use ecow::EcoString;

    #[test]
    fn plain_string_function_protocol_executes_graph_entries() {
        let source = r#"
fn identity(value: String) {
  value
}

pub fn main() {
  identity("one")
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::String(StringFunctionId(1))),
            [ParamLocal::String(StringLocalId(0))],
        );
        assert_eq!(
            run_main(&execution, &mut Vec::new()),
            Ok(Value::String("one".into())),
        );
    }

    #[test]
    fn hosted_string_function_protocol_executes_graph_and_host_entries() {
        let text = HostModule::new("host_support", "host/text")
            .expect("host module should be valid")
            .with_function("identity", |value: EcoString| value)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([text]).expect("host modules should be unique");
        let source = r#"
import host/text

fn identity(value: String) {
  value
}

pub fn main() {
  identity(text.identity("one"))
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
                .function(&FunctionTarget::String(StringFunctionId(2))),
            [ParamLocal::String(StringLocalId(0))],
        );
        assert_eq!(
            execution
                .execution()
                .function_parameters()
                .function(&FunctionTarget::String(StringFunctionId(1))),
            [ParamLocal::String(StringLocalId(0))],
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
            Ok(Value::String("one".into())),
        );
    }
}
