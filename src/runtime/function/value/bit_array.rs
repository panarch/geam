use super::super::{EvaluatedFunctionExit, evaluate_entry, parameter_locals};
use crate::plan::execution::function::BitArrayFunctionId;
use crate::plan::execution::graph::ParamLocal;
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::EvaluatedBitArray;
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeStateFor;

pub(in crate::runtime) fn run_bit_array<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    mut function: BitArrayFunctionId,
    mut origin: HostCallOrigin,
    mut inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArray> {
    loop {
        let exit = evaluate_entry(
            plan,
            state,
            plan.bit_array_function(function),
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

pub(in crate::runtime) fn bit_array_parameter_locals<Plan: ExecutableRuntimePlan>(
    plan: &Plan,
    function: BitArrayFunctionId,
) -> Vec<ParamLocal> {
    parameter_locals(plan, plan.bit_array_function(function))
}

#[cfg(test)]
mod tests {
    use super::bit_array_parameter_locals;
    use crate::plan::execution::function::BitArrayFunctionId;
    use crate::plan::execution::graph::{BitArrayLocalId, ParamLocal};
    use crate::{
        BitArrayValue, HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource,
        Value, compile_typed_host_program, compile_typed_module, plan_host_program, plan_module,
        run_main,
    };

    #[test]
    fn plain_bit_array_function_protocol_executes_graph_entries() {
        let source = r#"
fn identity(value: BitArray) {
  value
}

pub fn main() {
  identity(<<1, 2>>)
}
"#;
        let typed =
            compile_typed_module("main", "main.gleam", source).expect("source should compile");
        let plan = plan_module(typed).expect("source should plan");
        let execution = crate::ExecutionPlan::from_module_plan(plan);
        assert_eq!(
            bit_array_parameter_locals(&execution, BitArrayFunctionId(1)),
            [ParamLocal::BitArray(BitArrayLocalId(0))],
        );
        assert_eq!(
            run_main(&execution, &mut Vec::new()),
            Ok(Value::BitArray(BitArrayValue::from_bytes(vec![1, 2]))),
        );
    }

    #[test]
    fn hosted_bit_array_function_protocol_executes_graph_and_host_entries() {
        let binaries = HostModule::new("host_support", "host/binaries")
            .expect("host module should be valid")
            .with_function("identity", |value: BitArrayValue| value)
            .expect("host function should be valid");
        let hosts = HostProviderSet::new([binaries]).expect("host modules should be unique");
        let source = r#"
import host/binaries

fn identity(value: BitArray) {
  value
}

pub fn main() {
  identity(binaries.identity(<<1, 2>>))
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
            bit_array_parameter_locals(&execution, BitArrayFunctionId(2)),
            [ParamLocal::BitArray(BitArrayLocalId(0))],
        );
        assert_eq!(
            bit_array_parameter_locals(&execution, BitArrayFunctionId(1)),
            [ParamLocal::BitArray(BitArrayLocalId(0))],
        );
        assert_eq!(
            execution.run_main(&mut (), &mut Vec::new()),
            Ok(Value::BitArray(BitArrayValue::from_bytes(vec![1, 2]))),
        );
    }
}
