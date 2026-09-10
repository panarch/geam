use super::super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::UtfCodepointFunctionId;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn run_utf_codepoint(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: UtfCodepointFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<char> {
    run(plan, state, function, origin, inputs)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::function::UtfCodepointFunctionId;
    use crate::plan::execution::graph::FunctionTarget;
    use crate::plan::execution::graph::{ParamLocal, UtfCodepointLocalId};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, compile_typed_module, plan_host_program, plan_module, run_main,
    };

    #[test]
    fn plain_utf_codepoint_function_protocol_executes_graph_entries() {
        let source = r#"
fn identity(value: UtfCodepoint) {
  value
}

pub fn main() {
  let assert <<value:utf8_codepoint>> = <<"A":utf8>>
  identity(value)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::UtfCodepoint(UtfCodepointFunctionId(1))),
            [ParamLocal::UtfCodepoint(UtfCodepointLocalId(0))],
        );
        assert_eq!(
            run_main(&execution, &mut Vec::new()),
            Ok(Value::UtfCodepoint('A')),
        );
    }

    #[test]
    fn hosted_utf_codepoint_function_protocol_executes_graph_and_host_entries() {
        let codepoints = HostModule::new("host_support", "host/codepoints")
            .expect("host module should be valid")
            .with_function("identity", |value: char| value)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([codepoints]).expect("host modules should be unique");
        let source = r#"
import host/codepoints

fn identity(value: UtfCodepoint) {
  value
}

pub fn main() {
  let assert <<value:utf8_codepoint>> = <<"A":utf8>>
  identity(codepoints.identity(value))
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
                .function(&FunctionTarget::UtfCodepoint(UtfCodepointFunctionId(2))),
            [ParamLocal::UtfCodepoint(UtfCodepointLocalId(0))],
        );
        assert_eq!(
            execution
                .execution()
                .function_parameters()
                .function(&FunctionTarget::UtfCodepoint(UtfCodepointFunctionId(1))),
            [ParamLocal::UtfCodepoint(UtfCodepointLocalId(0))],
        );
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
            Ok(Value::UtfCodepoint('A')),
        );
    }
}
