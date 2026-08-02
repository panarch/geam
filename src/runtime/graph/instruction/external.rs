use super::super::environment::BlockEnvironment;
use super::value::{custom_projection, inputs_with_captures, list_element, tuple_projection};
use crate::plan::ValueType;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{ExternalInstructionRef, ExternalInstructionView};
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{EvaluatedExternalValue, EvaluatedValue};
use crate::runtime::state::RuntimeStateFor;
use crate::runtime::{ExecutableRuntimePlan, RuntimeGraph};

pub(super) fn evaluate<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    environment: &BlockEnvironment,
    instruction: &<RuntimeGraph<Plan> as ExecutionGraphProfile>::ExternalInstruction,
    expected: &ValueType,
) -> ExecutionResult<EvaluatedExternalValue>
where
    Plan: ExecutableRuntimePlan,
{
    match instruction.instruction_ref() {
        ExternalInstructionRef::Call {
            function,
            args,
            site,
        } => crate::runtime::function::run_external(
            plan,
            state,
            RuntimeGraph::<Plan>::external_function(function),
            crate::runtime::error::HostCallOrigin::source(site.to_owned()),
            environment.retain(args),
        ),
        ExternalInstructionRef::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = environment.external_function(function);
            crate::runtime::function::run_external(
                plan,
                state,
                function.runtime_id(),
                crate::runtime::error::HostCallOrigin::source(site.to_owned()),
                inputs_with_captures(environment, args, function.captures()),
            )
        }
        ExternalInstructionRef::TupleIndex { tuple, index } => tuple_projection(
            plan.value_metadata(),
            environment,
            tuple,
            index,
            expected,
            external_value,
        ),
        ExternalInstructionRef::CustomField { source, index } => {
            custom_projection(plan, environment, source, index, expected, external_value)
        }
        ExternalInstructionRef::ListIndex { list, index } => {
            let list = environment.external_list(list);
            let values = state.lists().external_values(&list);
            list_element(expected, index, &values)
        }
    }
}

fn external_value(value: &EvaluatedValue) -> Option<EvaluatedExternalValue> {
    let EvaluatedValue::External(value) = value else {
        return None;
    };
    Some(value.clone())
}

#[cfg(test)]
mod tests {
    use super::external_value;
    use crate::host::HostExternalStore;
    use crate::plan::execution::type_::ExternalTypeId;
    use crate::runtime::{EvaluatedExternalValue, EvaluatedValue};

    #[test]
    fn extracts_external_instruction_values() {
        let store = HostExternalStore::default();
        let source_equal =
            |context: &crate::host::HostExternalEquality<'_>,
             left: &crate::host::HostStoredValue<num_bigint::BigInt>,
             right: &crate::host::HostStoredValue<num_bigint::BigInt>| {
                context.stored_values_equal(left, right)
            };
        let lease = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            7,
            "7".into(),
        );
        let equal = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            7,
            "7".into(),
        );
        let stored_equal =
            |left: &crate::runtime::StoredRuntimeValue,
             right: &crate::runtime::StoredRuntimeValue| left.value() == right.value();
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        assert!(lease.source_equal(&equality, &equal));
        let expected = EvaluatedExternalValue::new(ExternalTypeId::new(0), lease);

        assert_eq!(expected.lease().inspect(), "7");
        assert_eq!(
            external_value(&EvaluatedValue::External(expected.clone())),
            Some(expected),
        );
    }

    #[test]
    fn rejects_non_external_instruction_values() {
        assert_eq!(external_value(&EvaluatedValue::Bool(true)), None,);
    }
}
