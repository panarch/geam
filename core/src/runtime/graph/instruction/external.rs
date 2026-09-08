use super::super::environment::BlockEnvironment;
use super::InstructionValueWithoutConstant;
use super::value::{custom_projection, inputs_with_captures, list_element, tuple_projection};
use crate::plan::ValueType;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{ExternalInstructionRef, ExternalInstructionView};
use crate::runtime::error::ExecutionResult;
use crate::runtime::evaluated::{EvaluatedExternalValue, EvaluatedValue};
use crate::runtime::graph::RuntimeGraphState;
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
    evaluate_action(plan, state, environment, instruction, expected).and_then(|action| match action
    {
        InstructionValueWithoutConstant::Ready(value) => Ok(value),
        InstructionValueWithoutConstant::Call {
            function,
            origin,
            inputs,
        } => crate::runtime::function::run_external(plan, state, function, origin, inputs),
    })
}

pub(in crate::runtime) fn evaluate_action<Plan, State>(
    plan: &Plan,
    state: &State,
    environment: &BlockEnvironment,
    instruction: &<RuntimeGraph<Plan> as ExecutionGraphProfile>::ExternalInstruction,
    expected: &ValueType,
) -> Result<
    InstructionValueWithoutConstant<
        EvaluatedExternalValue,
        crate::plan::execution::function::ExternalFunctionId,
    >,
    State::Error,
>
where
    Plan: crate::plan::execution::runtime::RuntimeExecutionPlan,
    State: RuntimeGraphState,
{
    use InstructionValueWithoutConstant as V;

    match instruction.instruction_ref() {
        ExternalInstructionRef::Call {
            function,
            args,
            site,
        } => Ok(V::Call {
            function: RuntimeGraph::<Plan>::external_function(function),
            origin: crate::runtime::error::HostCallOrigin::source(site.to_owned()),
            inputs: environment.retain(args),
        }),
        ExternalInstructionRef::FunctionCall {
            function,
            args,
            site,
        } => {
            let function = environment.external_function(function);
            Ok(V::Call {
                function: function.runtime_id(),
                origin: crate::runtime::error::HostCallOrigin::source(site.to_owned()),
                inputs: inputs_with_captures(environment, args, function.captures()),
            })
        }
        ExternalInstructionRef::TupleIndex { tuple, index } => tuple_projection(
            plan.value_metadata(),
            environment,
            tuple,
            index,
            expected,
            external_value,
        )
        .map(V::Ready),
        ExternalInstructionRef::CustomField { source, index } => {
            custom_projection(plan, environment, source, index, expected, external_value)
                .map(V::Ready)
        }
        ExternalInstructionRef::ListIndex { list, index } => {
            let list = environment.external_list(list);
            let values = state.lists().external_values(&list);
            list_element(expected, index, &values).map(V::Ready)
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
        fn source_hash(
            context: &crate::host::HostExternalHashing<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> u64 {
            context.stored_value_hash(value)
        }

        fn inspect(
            context: &crate::host::HostExternalInspection<'_>,
            value: &crate::host::HostStoredValue<num_bigint::BigInt>,
        ) -> ecow::EcoString {
            context.inspect_stored_value(value)
        }

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
            source_hash,
            inspect,
        );
        let equal = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            source_hash,
            inspect,
        );
        let stored_equal = |left: &crate::runtime::RetainedValueRef,
                            right: &crate::runtime::RetainedValueRef| {
            left.value() == right.value()
        };
        let equality = crate::host::RetainedValueEquality::new(&stored_equal);
        assert!(lease.source_equal(&equality, &equal));
        let expected: EvaluatedExternalValue =
            EvaluatedExternalValue::new(ExternalTypeId::new(0), lease);
        let stored_inspect = |_: &crate::runtime::RetainedValueRef| "7".into();
        let inspection = crate::host::RetainedValueInspection::new(&stored_inspect);

        let stored_hash = |_: &crate::runtime::RetainedValueRef| 7;
        let hashing = crate::host::RetainedValueHashing::new(&stored_hash);
        assert_eq!(expected.source_hash(&hashing), 7);
        assert_eq!(expected.lease().inspection(&inspection), "7");
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
