use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::error::HostCallOrigin;
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedCustomValue, EvaluatedExternalFunction, EvaluatedFloatFunction,
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedListFunction,
    EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::graph::RetainedValues;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum InvocableFunctionValue {
    Never(EvaluatedNeverFunction),
    Int(EvaluatedIntFunction),
    Float(EvaluatedFloatFunction),
    String(EvaluatedStringFunction),
    BitArray(EvaluatedBitArrayFunction),
    UtfCodepoint(EvaluatedUtfCodepointFunction),
    Custom(EvaluatedCustomFunction),
    External(EvaluatedExternalFunction),
    Bool(EvaluatedBoolFunction),
    Nil(EvaluatedNilFunction),
    Tuple(EvaluatedTupleFunction),
    List(EvaluatedListFunction),
    Function(EvaluatedFunctionFunction),
}

impl InvocableFunctionValue {
    pub(in crate::runtime) fn into_evaluated(self) -> EvaluatedFunctionValue {
        match self {
            Self::Never(function) => function.into(),
            Self::Int(function) => function.into(),
            Self::Float(function) => function.into(),
            Self::String(function) => function.into(),
            Self::BitArray(function) => function.into(),
            Self::UtfCodepoint(function) => function.into(),
            Self::Custom(function) => function.into(),
            Self::External(function) => function.into(),
            Self::Bool(function) => function.into(),
            Self::Nil(function) => function.into(),
            Self::Tuple(function) => function.into(),
            Self::List(function) => function.into(),
            Self::Function(function) => function.into(),
        }
    }
}

pub(in crate::runtime) fn prepare_callable<'plan, Plan: ExecutableRuntimePlan>(
    plan: &'plan Plan,
    function: &InvocableFunctionValue,
    origin: HostCallOrigin,
    arguments: Box<[EvaluatedValue]>,
) -> crate::runtime::execution::Invocation<'plan, Plan, EvaluatedValue> {
    use crate::plan::execution::function::{
        ListFunctionId, ProfiledFunctionFunctionId, RuntimeListFunctionId,
    };
    use crate::runtime::execution::Invocation;
    use crate::runtime::state::list::ListValueId;

    macro_rules! source {
        ($function:expr, $map:expr) => {{
            let function = $function;
            let inputs = callable_inputs(arguments, function.captures());
            Invocation::execution(plan, function.runtime_id(), origin, inputs)
                .map(|value| Ok(($map)(value)))
        }};
    }

    match function {
        InvocableFunctionValue::Never(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            Invocation::execution(plan, function.runtime_id(), origin, inputs)
                .map(|never| match never {})
        }
        InvocableFunctionValue::Int(function) => source!(function, EvaluatedValue::Int),
        InvocableFunctionValue::Float(function) => source!(function, EvaluatedValue::Float),
        InvocableFunctionValue::String(function) => source!(function, EvaluatedValue::String),
        InvocableFunctionValue::BitArray(function) => source!(function, EvaluatedValue::BitArray),
        InvocableFunctionValue::UtfCodepoint(function) => {
            source!(function, EvaluatedValue::UtfCodepoint)
        }
        InvocableFunctionValue::Custom(function) => match function {
            EvaluatedCustomFunction::Function(function) => {
                source!(function, EvaluatedValue::Custom)
            }
            EvaluatedCustomFunction::Constructor(function) => {
                Invocation::ready(EvaluatedValue::Custom(EvaluatedCustomValue::from_fields(
                    function.runtime_id(),
                    arguments,
                )))
            }
        },
        InvocableFunctionValue::External(function) => source!(function, EvaluatedValue::External),
        InvocableFunctionValue::Bool(function) => source!(function, EvaluatedValue::Bool),
        InvocableFunctionValue::Nil(function) => source!(function, |()| EvaluatedValue::Nil),
        InvocableFunctionValue::Tuple(function) => source!(function, EvaluatedValue::Tuple),
        InvocableFunctionValue::List(function) => {
            let inputs = callable_inputs(arguments, function.captures());
            macro_rules! list {
                ($id:expr, $variant:ident) => {
                    Invocation::execution(plan, $id, origin, inputs)
                        .map(|value| Ok(EvaluatedValue::from(ListValueId::$variant(value))))
                };
            }
            match function.runtime_id() {
                RuntimeListFunctionId::External(id) => list!(id, External),
                RuntimeListFunctionId::Core(id) => match id {
                    ListFunctionId::Parameter(id) => list!(id, Parameter),
                    ListFunctionId::ParameterList(id) => list!(id, ParameterList),
                    ListFunctionId::Int(id) => list!(id, Int),
                    ListFunctionId::Float(id) => list!(id, Float),
                    ListFunctionId::String(id) => list!(id, String),
                    ListFunctionId::BitArray(id) => list!(id, BitArray),
                    ListFunctionId::UtfCodepoint(id) => list!(id, UtfCodepoint),
                    ListFunctionId::Custom(id) => list!(id, Custom),
                    ListFunctionId::Bool(id) => list!(id, Bool),
                    ListFunctionId::Nil(id) => list!(id, Nil),
                    ListFunctionId::Tuple(id) => list!(id, Tuple),
                    ListFunctionId::List(id) => list!(id, List),
                    ListFunctionId::Function(id) => list!(id, Function),
                },
            }
        }
        InvocableFunctionValue::Function(function) => {
            macro_rules! returned_function {
                ($id:expr, $inputs:expr) => {
                    Invocation::execution(plan, $id, origin, $inputs)
                        .map(|value| Ok(EvaluatedValue::Function(value.into())))
                };
            }
            match function {
                EvaluatedFunctionFunction::Core(function) => {
                    let inputs = callable_inputs(arguments, function.captures());
                    match function.runtime_id() {
                        ProfiledFunctionFunctionId::Generic(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Never(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Int(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Float(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::String(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::BitArray(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::UtfCodepoint(id) => {
                            returned_function!(id, inputs)
                        }
                        ProfiledFunctionFunctionId::Custom(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Bool(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Nil(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Tuple(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::List(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::Function(id) => returned_function!(id, inputs),
                        ProfiledFunctionFunctionId::External(never) => match never {},
                    }
                }
                EvaluatedFunctionFunction::External(function) => {
                    let inputs = callable_inputs(arguments, function.captures());
                    match function.runtime_id() {
                        crate::plan::execution::graph::ExternalFunctionCallTarget::Function(id) => returned_function!(id, inputs),
                        crate::plan::execution::graph::ExternalFunctionCallTarget::ListFunction { id, .. } => returned_function!(id, inputs),
                    }
                }
            }
        }
    }
}

pub(in crate::runtime) fn callable_inputs(
    arguments: Box<[EvaluatedValue]>,
    captures: &[crate::runtime::evaluated::EvaluatedCapture],
) -> RetainedValues {
    let mut inputs = RetainedValues::empty();
    for value in arguments {
        inputs.push_evaluated(value);
    }
    inputs.append_captures(captures);
    inputs
}

#[cfg(test)]
mod tests {
    use super::{InvocableFunctionValue, prepare_callable};
    use crate::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use crate::execution_fixture::TestHost;
    use crate::host::{
        HostComponentProfile, HostFutureStore, HostProfile, HostProviderSet, HostWorkProfile,
    };
    use crate::work_fixture::{WorkComponent, WorkType};
    use crate::{ModuleSource, PackageSource};
    use num_bigint::BigInt;

    fn never_callable(value: crate::runtime::EvaluatedValue) -> InvocableFunctionValue {
        match value {
            crate::runtime::EvaluatedValue::Function(function) => match function.into_kind() {
                crate::runtime::evaluated::EvaluatedFunctionValueKind::Never(function) => {
                    InvocableFunctionValue::Never(function)
                }
                _ => panic!("the source must return a non-returning callback"),
            },
            _ => panic!("the source must return a callback"),
        }
    }

    fn invoke_non_returning_entry(source: &str) -> crate::ExecutionError {
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::<Profile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let plan = execution.execution();
        let services = crate::runtime::execution::Services::new();
        let context = services.context();
        let host = TestHost::default();
        let value = host
            .block_on(
                super::super::prepare_main(plan).submit(&context, std::num::NonZeroUsize::MIN),
            )
            .unwrap()
            .unwrap();
        let callback = never_callable(value);
        host.block_on(
            prepare_callable(
                plan,
                &callback,
                crate::runtime::HostCallOrigin::Entry,
                Box::new([crate::runtime::EvaluatedValue::Int(42.into())]),
            )
            .submit(&context, std::num::NonZeroUsize::MIN),
        )
        .unwrap()
        .unwrap_err()
    }

    #[test]
    fn non_returning_callbacks_preserve_their_source_failure_without_a_result_storage() {
        let error = invoke_non_returning_entry(
            "pub fn main() { fn(_value: Int) { panic as \"callback stopped\" } }",
        );
        assert_eq!(error.to_string(), "panic: callback stopped");
    }

    #[test]
    #[should_panic(expected = "the source must return a callback")]
    fn non_returning_callback_fixture_rejects_a_scalar() {
        invoke_non_returning_entry("pub fn main() { 42 }");
    }

    #[test]
    #[should_panic(expected = "the source must return a non-returning callback")]
    fn non_returning_callback_fixture_rejects_a_returning_function() {
        invoke_non_returning_entry("pub fn main() { fn(value: Int) { value } }");
    }

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn deferred_callbacks_preserve_value_families_and_captures() {
        let mut state = ();
        assert!(std::ptr::eq(Profile::component_state(&mut state), &state));
        for (source, producer, expected) in [
            "fn produce(_) { 42 } fn consume(value) { value }",
            "fn produce(_) { 1.5 } fn consume(value) { let assert 1.5 = value 42 }",
            "fn produce(_) { \"text\" } fn consume(value) { let assert \"text\" = value 42 }",
            "fn produce(_) { <<42>> } fn consume(value) { let assert <<42>> = value 42 }",
            "fn produce(_) { let assert <<c:utf8_codepoint>> = <<65>> c } fn consume(value) { let assert <<65>> = <<value:utf8_codepoint>> 42 }",
            "type Item { Item(Int) } fn produce(_) { Item(42) } fn consume(item) { let Item(value) = item value }",
            "fn produce(_) { future.ready(42) } fn consume(value) { let assert True = value == value 42 }",
            "fn produce(_) { True } fn consume(value) { let assert True = value 42 }",
            "fn produce(_) { Nil } fn consume(value) { let Nil = value 42 }",
            "fn produce(_) { #(40, 2) } fn consume(value) { let #(a, b) = value a + b }",
            "fn produce(_) -> List(a) { [] } fn consume(value) { let assert [] = value 42 }",
            "fn produce(_) -> List(List(a)) { [[]] } fn consume(value) { let assert [[]] = value 42 }",
            "fn produce(_) { [40, 2] } fn consume(value) { let assert [a, b] = value a + b }",
            "fn produce(_) { [1.5] } fn consume(value) { let assert [1.5] = value 42 }",
            "fn produce(_) { [\"text\"] } fn consume(value) { let assert [\"text\"] = value 42 }",
            "fn produce(_) { [<<42>>] } fn consume(value) { let assert [<<42>>] = value 42 }",
            "fn produce(_) { let assert <<c:utf8_codepoint>> = <<65>> [c] } fn consume(value) { let assert [c] = value let assert <<65>> = <<c:utf8_codepoint>> 42 }",
            "type Item { Item(Int) } fn produce(_) { [Item(42)] } fn consume(value) { let assert [Item(number)] = value number }",
            "fn produce(_) { [future.ready(42)] } fn consume(value) { let assert [work] = value let assert True = work == work 42 }",
            "fn produce(_) { [True] } fn consume(value) { let assert [True] = value 42 }",
            "fn produce(_) { [Nil] } fn consume(value) { let assert [Nil] = value 42 }",
            "fn produce(_) { [#(40, 2)] } fn consume(value) { let assert [#(a, b)] = value a + b }",
            "fn produce(_) { [[40, 2]] } fn consume(value) { let assert [[a, b]] = value a + b }",
            "fn produce(_) { [fn() { 42 }] } fn consume(value) { let assert [callback] = value callback() }",
            "fn produce(_) { fn(value) { value } } fn consume(callback) { callback(42) }",
            "fn produce(_) { fn(value) { value } } fn consume(_) { 42 }",
            "fn produce(_) { fn() { panic } } fn consume(_) { 42 }",
            "fn produce(_) { let captured = 40 fn() { captured + 2 } } fn consume(callback) { callback() }",
            "fn produce(_) { fn() { 1.5 } } fn consume(callback) { let assert 1.5 = callback() 42 }",
            "fn produce(_) { fn() { \"text\" } } fn consume(callback) { let assert \"text\" = callback() 42 }",
            "fn produce(_) { fn() { <<42>> } } fn consume(callback) { let assert <<42>> = callback() 42 }",
            "fn produce(_) { fn() { let assert <<c:utf8_codepoint>> = <<65>> c } } fn consume(callback) { let c = callback() let assert <<65>> = <<c:utf8_codepoint>> 42 }",
            "type Item { Item(Nil, Int) } fn produce(_) { fn() { Item(Nil, 42) } } fn consume(callback) { let Item(Nil, value) = callback() value }",
            "fn produce(_) { fn() { True } } fn consume(callback) { let assert True = callback() 42 }",
            "fn produce(_) { fn() { Nil } } fn consume(callback) { let Nil = callback() 42 }",
            "fn produce(_) { fn() { #(40, 2) } } fn consume(callback) { let #(a, b) = callback() a + b }",
            "fn produce(_) { fn() { [40, 2] } } fn consume(callback) { let assert [a, b] = callback() a + b }",
            "fn produce(_) { fn() { fn() { 42 } } } fn consume(callback) { let inner = callback() inner() }",
            "fn produce(_) { fn() { future.ready(1) } } fn consume(callback) { let work = callback() let assert True = work == work 42 }",
            "fn produce(_) { fn() { [future.ready(1)] } } fn consume(callback) { let assert [work] = callback() let assert True = work == work 42 }",
        ].into_iter().map(|source| (source, "produce", Ok(42))).chain([
            ("type Item { Item(Nil) } fn consume(item) { let Item(Nil) = item 42 }", "Item", Ok(42)),
            ("fn produce(_) { panic as \"deferred callback failed\" } fn consume(value: Int) { value }", "produce", Err("panic: deferred callback failed")),
        ]) {
            let source = format!(
                r#"
import fixture/work as future
{source}
pub fn run() {{
  let returned = future.map(future.ready(Nil), {producer})
  future.map(returned, consume)
}}
"#
            );
            let typed = crate::compile_typed_host_program(
                "application",
                "library",
                [
                    PackageSource::new(
                        "work_fixture",
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "fixture/work",
                            "fixture/work.gleam",
                            WorkComponent::SOURCE,
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        ["work_fixture"],
                        [ModuleSource::new("library", "library.gleam", &source)],
                    ),
                ],
                HostProviderSet::from_providers(WorkComponent::providers::<Profile>().unwrap())
                    .unwrap(),
            )
            .expect(&source);
            let (bindings, run) = HostedModuleBuilder::new(typed)
                .expect(&source)
                .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("run"))
                .unwrap();
            let mut module = bindings.seal().unwrap();
            let host = TestHost::default();
            let mut echo = Vec::new();
            host.block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    let work = scope.call(&run, ()).await.unwrap();
                    let result = scope.observe(&work).await
                        .map(|completed| completed.read(Clone::clone))
                        .map_err(|error| error.to_string());
                    assert_eq!(result, expected.map(BigInt::from).map_err(str::to_owned), "{source}");
                }),
            )
            .unwrap();
            assert!(echo.is_empty());
        }
    }
}
