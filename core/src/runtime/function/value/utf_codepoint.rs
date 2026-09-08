use super::super::{EvaluatedFunctionExit, evaluate_entry};
use crate::plan::execution::function::UtfCodepointFunctionId;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::ProfiledRetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_utf_codepoint<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: UtfCodepointFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: ProfiledRetainedValues<Plan::Values>,
) -> ExecutionResult<char, Plan::Values> {
    loop {
        let exit = evaluate_entry(
            plan,
            state,
            plan.utf_codepoint_function(function),
            origin,
            inputs,
        )?;
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
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("hosted execution should seal");
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::UtfCodepoint(UtfCodepointFunctionId(2))),
            [ParamLocal::UtfCodepoint(UtfCodepointLocalId(0))],
        );
        assert_eq!(
            execution
                .function_parameters()
                .function(&FunctionTarget::UtfCodepoint(UtfCodepointFunctionId(1))),
            [ParamLocal::UtfCodepoint(UtfCodepointLocalId(0))],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::UtfCodepoint('A')),
        );
    }
}
