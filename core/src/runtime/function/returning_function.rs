use super::run;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, CustomFunctionFunctionId,
    FloatFunctionFunctionId, FunctionFunctionFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, NeverFunctionFunctionId, NilFunctionFunctionId,
    ParameterFunctionFunctionId, ProfiledFunctionFunctionId, ProfiledListFunctionFunctionId,
    StringFunctionFunctionId, TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedFloatFunction, EvaluatedFunctionFunction, EvaluatedFunctionValue,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedParameterFunction, EvaluatedStringFunction,
    EvaluatedTupleFunction, EvaluatedUtfCodepointFunction,
};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::RuntimeState;
use std::convert::Infallible;

pub(in crate::runtime) fn run_core_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ProfiledFunctionFunctionId<Infallible>,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionValue> {
    use ProfiledFunctionFunctionId as F;

    match function {
        F::Generic(function) => {
            run_generic_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Never(function) => {
            run_never_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Int(function) => run_int_function(plan, state, function, origin, inputs).map(Into::into),
        F::Float(function) => {
            run_float_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::String(function) => {
            run_string_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::BitArray(function) => {
            run_bit_array_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::UtfCodepoint(function) => {
            run_utf_codepoint_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Parameter(function) => {
            run_parameter_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Custom(function) => {
            run_custom_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::External(function) => match function {},
        F::Bool(function) => {
            run_bool_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Nil(function) => run_nil_function(plan, state, function, origin, inputs).map(Into::into),
        F::Tuple(function) => {
            run_tuple_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::List(function) => {
            run_core_list_function(plan, state, function, origin, inputs).map(Into::into)
        }
        F::Function(function) => {
            run_function_function(plan, state, function, origin, inputs).map(Into::into)
        }
    }
}

pub(in crate::runtime) fn run_generic_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: GenericFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedGenericFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_never_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NeverFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedNeverFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_int_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: IntFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedIntFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_float_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: FloatFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFloatFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_string_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: StringFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedStringFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_bit_array_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BitArrayFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBitArrayFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_utf_codepoint_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: UtfCodepointFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedUtfCodepointFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_parameter_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ParameterFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedParameterFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_custom_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: CustomFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedCustomFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_bool_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: BoolFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedBoolFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_nil_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: NilFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedNilFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_tuple_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: TupleFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedTupleFunction> {
    run(plan, state, function, origin, inputs)
}

fn run_core_list_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: ProfiledListFunctionFunctionId<Infallible>,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedListFunction> {
    run(plan, state, function, origin, inputs)
}

pub(in crate::runtime) fn run_function_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState<'_>,
    function: FunctionFunctionFunctionId,
    origin: HostCallOrigin,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedFunctionFunction> {
    run(plan, state, function, origin, inputs)
}

#[cfg(test)]
mod tests {
    use crate::plan::{CustomType, CustomTypeName, FunctionType, TypeParameterId, ValueType};

    #[test]
    fn general_dispatch_returns_and_retains_every_core_callable_family_with_its_exact_type() {
        use crate::plan::execution::function::{
            ProfiledCoreRuntimeFunctionId, ProfiledRuntimeFunctionId, RuntimeFunctionFunctionTarget,
        };
        use crate::plan::execution::runtime::RuntimeExecutionPlan;
        use crate::runtime::evaluated::EvaluatedValueRef;
        use crate::runtime::state::RuntimeState;
        use crate::runtime::{EvaluatedValue, HostCallOrigin, RetainedValues, StoredRuntimeValue};

        let empty = ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Empty".into()),
            Vec::new(),
        ));
        let function = |arguments, return_| {
            Some(ValueType::Function(Box::new(FunctionType::new(
                arguments, return_,
            ))))
        };
        for (source, expected) in [
            (
                "pub type Empty pub fn main() -> fn(Empty) -> Int { fn(_) { 42 } }",
                Some(ValueType::Function(Box::new(FunctionType::new(
                    vec![empty.clone()],
                    ValueType::Int,
                )))),
            ),
            (
                "pub fn main() { fn() { 42 } }",
                function(vec![], ValueType::Int),
            ),
            (
                "pub fn main() { fn() { 1.5 } }",
                function(vec![], ValueType::Float),
            ),
            (
                "pub fn main() { fn() { \"text\" } }",
                function(vec![], ValueType::String),
            ),
            (
                "pub fn main() { fn() { <<42>> } }",
                function(vec![], ValueType::BitArray),
            ),
            (
                "pub fn main() { fn() { let assert <<value:utf8_codepoint>> = <<65>> value } }",
                function(vec![], ValueType::UtfCodepoint),
            ),
            (
                "pub fn main() { fn() { True } }",
                function(vec![], ValueType::Bool),
            ),
            (
                "pub fn main() { fn() { Nil } }",
                function(vec![], ValueType::Nil),
            ),
            (
                "pub fn main() { fn() { #(42) } }",
                function(vec![], ValueType::Tuple(vec![ValueType::Int])),
            ),
            (
                "pub fn main() { fn() { [42] } }",
                function(vec![], ValueType::List(Box::new(ValueType::Int))),
            ),
            (
                "pub type Box { Box(Int) } pub fn main() { fn() { Box(42) } }",
                function(
                    vec![],
                    ValueType::Custom(CustomType::new(
                        CustomTypeName::new("geam".into(), "main".into(), "Box".into()),
                        Vec::new(),
                    )),
                ),
            ),
            (
                "pub fn main() { fn() { fn(value: Int) { value } } }",
                function(
                    vec![],
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            ),
            (
                "pub type Empty pub fn main() -> fn() -> Empty { fn() { panic } }",
                function(vec![], empty),
            ),
            (
                "pub fn main() { fn(value) { value } }",
                function(
                    vec![ValueType::Parameter(TypeParameterId::scoped(0, 0))],
                    ValueType::Parameter(TypeParameterId::scoped(0, 0)),
                ),
            ),
            ("pub fn main() { 42 }", None),
        ] {
            let plan = crate::runtime::plan_src(source);
            let mut echo = Vec::new();
            let mut state = RuntimeState::new(&mut echo);
            let value = match plan.main_runtime() {
                ProfiledRuntimeFunctionId::Core(ProfiledCoreRuntimeFunctionId::Function {
                    id: RuntimeFunctionFunctionTarget::Core(id),
                    ..
                }) => Some(EvaluatedValue::Function(
                    super::run_core_function(
                        &plan,
                        &mut state,
                        id,
                        HostCallOrigin::Entry,
                        RetainedValues::empty(),
                    )
                    .unwrap(),
                )),
                _ => None,
            };
            let actual = value.map(|value| {
                let retained = EvaluatedValueRef::from(&value).retain();
                assert_eq!(retained, value);
                StoredRuntimeValue::new(retained, plan.value_metadata())
                    .type_()
                    .clone()
            });
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn returned_symbolic_never_and_parameter_functions_keep_distinct_source_types() {
        let empty = ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Empty".into()),
            Vec::new(),
        ));
        let parameter = ValueType::Parameter(TypeParameterId::scoped(0, 0));
        for (source, argument, result) in [
            (
                "pub type Empty pub fn main() -> fn(Empty) -> Int { fn(_) { 42 } }",
                empty.clone(),
                ValueType::Int,
            ),
            (
                "pub type Empty pub fn main() -> fn(Int) -> Empty { fn(_) { panic } }",
                ValueType::Int,
                empty,
            ),
            (
                "pub fn main() { fn(value: a) { value } }",
                parameter.clone(),
                parameter,
            ),
        ] {
            let value = crate::runtime::run_src(source);
            assert_eq!(value.inspect().to_string(), "//fn(a) { ... }");
            assert_eq!(
                value.value_type(),
                ValueType::Function(Box::new(FunctionType::new(vec![argument], result))),
            );
            assert_eq!(value, value.clone());
        }
    }
}
