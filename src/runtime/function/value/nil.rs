use super::super::{EvaluatedFunctionExit, evaluate};
use crate::plan::execution::function::{
    ExecutionFunctionEntry, ExecutionFunctionRef, NilFunctionId,
};
use crate::plan::execution::graph::ParamLocal;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_nil<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: NilFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<()> {
    loop {
        let exit = match plan.nil_function(function).as_ref() {
            ExecutionFunctionRef::Graph(function) => evaluate(plan, state, function.body(), inputs),
            ExecutionFunctionRef::Host(target) => plan
                .call_host_nil(state, origin.clone(), target, &inputs)
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

pub(in crate::runtime) fn nil_parameter_locals<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    function: NilFunctionId,
) -> Vec<ParamLocal> {
    match plan.nil_function(function).as_ref() {
        ExecutionFunctionRef::Graph(function) => function
            .entry()
            .params(function.body())
            .iter()
            .map(|slot| slot.local().clone())
            .collect(),
        ExecutionFunctionRef::Host(target) => plan.host_nil_parameters(target).to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::nil_parameter_locals;
    use crate::plan::execution::function::NilFunctionId;
    use crate::plan::execution::graph::{NilLocalId, ParamLocal};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, compile_typed_module, plan_host_program, plan_module, run_main,
    };

    #[test]
    fn plain_nil_function_protocol_executes_graph_entries() {
        let source = r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity(Nil)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            nil_parameter_locals(&execution, NilFunctionId(1)),
            [ParamLocal::Nil(NilLocalId(0))],
        );
        assert_eq!(run_main(&execution, &mut Vec::new()), Ok(Value::Nil),);
    }

    #[test]
    fn hosted_nil_function_protocol_executes_graph_and_host_entries() {
        let values = HostModule::new("host_support", "host/values")
            .expect("host module should be valid")
            .with_function("identity", |(): ()| ())
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([values]).expect("host modules should be unique");
        let source = r#"
import host/values

fn identity(value: Nil) {
  value
}

pub fn main() {
  identity(values.identity(Nil))
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
            nil_parameter_locals(&execution, NilFunctionId(2)),
            [ParamLocal::Nil(NilLocalId(0))],
        );
        assert_eq!(
            nil_parameter_locals(&execution, NilFunctionId(1)),
            [ParamLocal::Nil(NilLocalId(0))],
        );
        assert_eq!(execution.run_main(&mut (), &mut Vec::new()), Ok(Value::Nil),);
    }
}
