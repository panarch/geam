use std::convert::Infallible;

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionGraphProfile, ExecutionHostTarget, ExecutionNeverHostTarget,
    ExecutionProfile,
};
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::execution::invocation::Waiting;
use crate::runtime::execution::{Invocation, ServiceContext};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state;
use crate::runtime::{graph, host};
use std::num::NonZeroUsize;

pub(in crate::runtime) type RuntimeGraph<Plan> =
    <<Plan as RuntimeExecutionPlan>::Profile as ExecutionProfile>::Graph;

pub(in crate::runtime) trait ExecutableRuntimePlan:
    RuntimeExecutionPlan + Sync
{
    type RuntimeHost<'run>: state::RuntimeHostState<State = Self::RunState>
    where
        Self: 'run;

    type HostInvocation<'plan, Output: Send + 'plan>: Send + 'plan
    where
        Self: 'plan;

    fn prepare_host<'plan, Body>(
        &'plan self,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> Self::HostInvocation<'plan, <Body::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue;

    fn prepare_host_never<'plan>(
        &'plan self,
        origin: HostCallOrigin,
        target: &ExecutionNeverHostTarget<Self::Profile>,
        inputs: RetainedValues,
    ) -> Self::HostInvocation<'plan, Infallible>;

    fn map_host<'plan, Input: Send + 'plan, Output: Send + 'plan>(
        invocation: Self::HostInvocation<'plan, Input>,
        map: impl FnOnce(Input) -> ExecutionResult<Output> + Send + 'plan,
    ) -> Self::HostInvocation<'plan, Output>;

    fn submit_host<'plan, Output: Send + 'plan>(
        invocation: Self::HostInvocation<'plan, Output>,
        context: &ServiceContext<Self>,
        budget: NonZeroUsize,
    ) -> Waiting<'plan, Output>;

    fn advance_external_list_instruction<'plan>(
        &'plan self,
        state: &mut impl graph::RuntimeGraphState<Error = crate::ExecutionError>,
        frame: graph::Frame<'plan, Self>,
        returns: &mut graph::Returns<'plan, Self>,
        instruction: &<RuntimeGraph<Self> as ExecutionGraphProfile>::ExternalListInstruction,
        expected: &crate::plan::ValueType,
    ) -> ExecutionResult<graph::Activation<'plan, Self>>;

    fn advance_external_function_instruction<'plan>(
        &'plan self,
        frame: graph::Frame<'plan, Self>,
        returns: &mut graph::Returns<'plan, Self>,
        instruction: &<RuntimeGraph<Self> as ExecutionGraphProfile>::ExternalFunctionInstruction,
    ) -> graph::Activation<'plan, Self>;
}

impl ExecutableRuntimePlan for ExecutionPlan {
    type RuntimeHost<'run> = ();
    type HostInvocation<'plan, Output: Send + 'plan> = Infallible;

    fn prepare_host<Body>(
        &self,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        _inputs: RetainedValues,
    ) -> Infallible
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue,
    {
        match *target {}
    }

    fn prepare_host_never(
        &self,
        _origin: HostCallOrigin,
        target: &ExecutionNeverHostTarget<Self::Profile>,
        _inputs: RetainedValues,
    ) -> Infallible {
        match *target {}
    }

    fn map_host<'plan, Input: Send + 'plan, Output: Send + 'plan>(
        invocation: Infallible,
        _map: impl FnOnce(Input) -> ExecutionResult<Output> + Send + 'plan,
    ) -> Infallible {
        match invocation {}
    }

    fn submit_host<'plan, Output: Send + 'plan>(
        invocation: Infallible,
        _context: &ServiceContext<Self>,
        _budget: NonZeroUsize,
    ) -> Waiting<'plan, Output> {
        match invocation {}
    }

    fn advance_external_list_instruction<'plan>(
        &'plan self,
        _state: &mut impl graph::RuntimeGraphState<Error = crate::ExecutionError>,
        _frame: graph::Frame<'plan, Self>,
        _returns: &mut graph::Returns<'plan, Self>,
        instruction: &Infallible,
        _expected: &crate::plan::ValueType,
    ) -> ExecutionResult<graph::Activation<'plan, Self>> {
        match *instruction {}
    }

    fn advance_external_function_instruction<'plan>(
        &'plan self,
        _frame: graph::Frame<'plan, Self>,
        _returns: &mut graph::Returns<'plan, Self>,
        instruction: &Infallible,
    ) -> graph::Activation<'plan, Self> {
        match *instruction {}
    }
}

impl<Profile: crate::HostProfile> ExecutableRuntimePlan
    for crate::plan::execution::HostedProgram<Profile>
{
    type RuntimeHost<'run>
        = crate::runtime::state::RuntimeHost<'run, Profile>
    where
        Self: 'run;

    type HostInvocation<'plan, Output: Send + 'plan> = Invocation<'plan, Self, Output>;

    fn prepare_host<'plan, Body>(
        &'plan self,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> Invocation<'plan, Self, <Body::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue,
    {
        let target = target.clone();
        Invocation::new(move |plan: &Self, state| match target {
            crate::plan::execution::host::HostedFunctionTarget::Value(target) => {
                host::invoke_value(plan, state, origin, &target, inputs)
            }
            crate::plan::execution::host::HostedFunctionTarget::Never(target) => {
                host::invoke_never(plan, state, origin, target, inputs).map(|never| match never {})
            }
        })
    }

    fn prepare_host_never<'plan>(
        &'plan self,
        origin: HostCallOrigin,
        target: &ExecutionNeverHostTarget<Self::Profile>,
        inputs: RetainedValues,
    ) -> Invocation<'plan, Self, Infallible> {
        let target = *target;
        Invocation::new(move |plan: &Self, state| {
            host::invoke_never(plan, state, origin, target, inputs).map(|never| match never {})
        })
    }

    fn map_host<'plan, Input: Send + 'plan, Output: Send + 'plan>(
        invocation: Invocation<'plan, Self, Input>,
        map: impl FnOnce(Input) -> ExecutionResult<Output> + Send + 'plan,
    ) -> Invocation<'plan, Self, Output> {
        invocation.map(map)
    }

    fn submit_host<'plan, Output: Send + 'plan>(
        invocation: Invocation<'plan, Self, Output>,
        context: &ServiceContext<Self>,
        budget: NonZeroUsize,
    ) -> Waiting<'plan, Output> {
        invocation.submit(context, budget)
    }

    fn advance_external_list_instruction<'plan>(
        &'plan self,
        state: &mut impl graph::RuntimeGraphState<Error = crate::ExecutionError>,
        frame: graph::Frame<'plan, Self>,
        returns: &mut graph::Returns<'plan, Self>,
        instruction: &crate::plan::execution::graph::ExternalListInstruction,
        expected: &crate::plan::ValueType,
    ) -> ExecutionResult<graph::Activation<'plan, Self>> {
        graph::advance_external_list_instruction(self, state, frame, returns, instruction, expected)
    }

    fn advance_external_function_instruction<'plan>(
        &'plan self,
        frame: graph::Frame<'plan, Self>,
        returns: &mut graph::Returns<'plan, Self>,
        instruction: &crate::plan::execution::graph::ExternalFunctionInstruction,
    ) -> graph::Activation<'plan, Self> {
        graph::advance_external_function_instruction(self, frame, returns, instruction)
    }
}

#[cfg(test)]
pub(in crate::runtime) mod external_test {
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, ExternalTestStores, HostExternalSchema,
        HostExternalStorage, HostExternalStore,
    };
    use crate::{
        HostExternalBinding, HostExternalType, HostFunctionType, HostProvider, HostTypeList,
        HostTypeListEnd, HostTypeParameter,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    pub(in crate::runtime) struct RuntimeCounterSchema;
    pub(in crate::runtime) struct RuntimeCounterProvider;
    pub(in crate::runtime) struct RuntimeCounterStorage;
    pub(in crate::runtime) type RuntimeHostCounter = HostExternalType<RuntimeCounterSchema>;
    pub(in crate::runtime) type RuntimeIntArguments = HostTypeList<BigInt, HostTypeListEnd>;
    pub(in crate::runtime) type RuntimeCounterCallable =
        HostFunctionType<RuntimeIntArguments, RuntimeHostCounter>;
    pub(in crate::runtime) type RuntimeGenericValue = HostTypeParameter<0>;
    pub(in crate::runtime) type RuntimeGenericCallable =
        HostFunctionType<HostTypeListEnd, RuntimeGenericValue>;

    impl HostExternalSchema for RuntimeCounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<ExternalTestProfile, RuntimeCounterSchema> for RuntimeCounterStorage {
        type Payload = BigInt;

        fn store(stores: &ExternalTestStores) -> &HostExternalStore<Self::Payload> {
            &stores.integers
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            left: &Self::Payload,
            right: &Self::Payload,
        ) -> bool {
            left == right
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, value: &Self::Payload) -> u64 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            std::hash::Hash::hash(value, &mut hasher);
            std::hash::Hasher::finish(&hasher)
        }

        fn inspect(
            _: &crate::host::HostExternalInspection<'_>,
            value: &Self::Payload,
        ) -> EcoString {
            format!("Counter({value})").into()
        }
    }

    impl HostProvider<ExternalTestProfile> for RuntimeCounterProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    impl HostExternalBinding<ExternalTestProfile, RuntimeCounterSchema> for RuntimeCounterProvider {
        type Storage = RuntimeCounterStorage;
    }

    #[test]
    fn runtime_counter_fixture_source_hash_is_exact() {
        let retained_hash = |_: &crate::runtime::RetainedValueRef| 0;
        let raw_hashing = crate::host::RetainedValueHashing::new(&retained_hash);
        let hashing = crate::host::HostExternalHashing(&raw_hashing);
        let value = BigInt::from(7);
        let mut expected = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&value, &mut expected);

        assert_eq!(
            <RuntimeCounterStorage as HostExternalStorage<
                ExternalTestProfile,
                RuntimeCounterSchema,
            >>::source_hash(&hashing, &value,),
            std::hash::Hasher::finish(&expected),
        );
    }
}

#[cfg(test)]
mod tests {

    use super::external_test::{
        RuntimeCounterCallable, RuntimeCounterProvider, RuntimeCounterSchema,
        RuntimeGenericCallable, RuntimeGenericValue, RuntimeHostCounter, RuntimeIntArguments,
    };
    use crate::host::{ExternalTestProfile, ExternalTestRunState};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostConstructions, HostExternal,
        HostListType, HostModule, HostProviderModule, HostProviderSet, HostTypeIndex0,
        HostTypeList, HostTypeListEnd, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn hosted_runtime_preserves_external_values_across_executable_owners() {
        fn new_counter<'call>(
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeHostCounter,
            >,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, RuntimeHostCounter>, HostCallError> {
            let _state = call.state();
            let counter = call.create_external(value);
            Ok(call.return_value(counter))
        }

        fn duplicate<'call>(
            call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                HostListType<RuntimeHostCounter>,
            >,
            counter: HostExternal<'call, RuntimeHostCounter>,
        ) -> Result<HostCallCompletion<'call, HostListType<RuntimeHostCounter>>, HostCallError>
        {
            Ok(call.return_list([counter, counter]))
        }

        fn construct_list_item<'call>(
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                HostListType<RuntimeHostCounter>,
            >,
            constructions: HostConstructions<
                'call,
                HostTypeList<RuntimeHostCounter, HostTypeListEnd>,
            >,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, HostListType<RuntimeHostCounter>>, HostCallError>
        {
            let counter = call.construct_external(constructions.at::<HostTypeIndex0>(), value);
            Ok(call.return_list([counter]))
        }

        fn invoke_counter<'call>(
            call: HostCall<'call, ExternalTestProfile, RuntimeCounterProvider, RuntimeHostCounter>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            function: HostCallable<'call, RuntimeIntArguments, RuntimeHostCounter>,
            value: BigInt,
        ) -> Result<crate::HostCallContinuation<'call, RuntimeHostCounter>, HostCallError> {
            type Owned = crate::provider::Value<
                RuntimeHostCounter,
                crate::provider::ProviderValueContext<RuntimeHostCounter>,
            >;
            let callback = call.owned_callable(function, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let counter = callback
                        .invoke(
                            &context,
                            move |_, _| (value, ()),
                            |call, _, value| Ok(Owned::from_host(&call, value)),
                        )
                        .await?;
                    Ok(crate::HostOwnedCompletion::new(move |mut call, _| {
                        let value = counter.into_host(&mut call);
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }

        fn invoke<'call>(
            call: HostCall<'call, ExternalTestProfile, RuntimeCounterProvider, RuntimeGenericValue>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            function: HostCallable<'call, HostTypeListEnd, RuntimeGenericValue>,
        ) -> Result<crate::HostCallContinuation<'call, RuntimeGenericValue>, HostCallError>
        {
            type Owned = crate::provider::Value<
                RuntimeGenericValue,
                crate::provider::ProviderValueContext<RuntimeGenericValue>,
            >;
            let callback = call.owned_callable(function, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let value = callback
                        .invoke(
                            &context,
                            |_, _| (),
                            |call, _, value| Ok(Owned::from_host(&call, value)),
                        )
                        .await?;
                    Ok(crate::HostOwnedCompletion::new(move |mut call, _| {
                        let value = value.into_host(&mut call);
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }

        let provider = || {
            HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<RuntimeCounterProvider, RuntimeCounterSchema>()
            .expect("external type should be valid")
            .with_scoped_function::<RuntimeCounterProvider, (BigInt,), RuntimeHostCounter, _>(
                "new_counter",
                new_counter,
            )
            .expect("external constructor should be valid")
            .with_scoped_function::<
                RuntimeCounterProvider,
                (RuntimeHostCounter,),
                HostListType<RuntimeHostCounter>,
                _,
            >(
                "duplicate",
                duplicate,
            )
            .expect("external list constructor should be valid")
            .with_scoped_function_and_constructions::<
                RuntimeCounterProvider,
                (BigInt,),
                HostListType<RuntimeHostCounter>,
                HostTypeList<RuntimeHostCounter, HostTypeListEnd>,
                _,
            >("construct_list_item", construct_list_item)
            .expect("intermediate external constructor should be valid")
            .with_resumable_function::<
                RuntimeCounterProvider,
                (RuntimeCounterCallable, BigInt),
                RuntimeHostCounter,
                HostTypeListEnd,
                _,
            >(
                "invoke_counter",
                invoke_counter,
            )
            .expect("external callback should be valid")
            .with_resumable_function::<
                RuntimeCounterProvider,
                (RuntimeGenericCallable,),
                RuntimeGenericValue,
                HostTypeListEnd,
                _,
            >("invoke", invoke)
            .expect("generic callback should be valid")
        };
        let source = r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "new_counter")
fn new_counter(value: Int) -> Counter

@external(erlang, "host", "duplicate")
fn duplicate(counter: Counter) -> List(Counter)

@external(erlang, "host", "construct_list_item")
fn construct_list_item(value: Int) -> List(Counter)

@external(erlang, "host", "invoke_counter")
fn invoke_counter(function: fn(Int) -> Counter, value: Int) -> Counter

@external(erlang, "host", "invoke")
fn invoke(function: fn() -> value) -> value

fn external_list(value: Int) -> List(Counter) {
  duplicate(new_counter(value))
}

fn external_function() -> fn(Int) -> Counter {
  new_counter
}

fn external_list_function() -> fn(Int) -> List(Counter) {
  external_list
}

fn function_list() -> List(fn(Int) -> Counter) {
  [new_counter]
}

fn function_function() -> fn() -> fn(Int) -> Counter {
  external_function
}

fn exercise() {
  let make = external_function()
  let make_list = external_list_function()
  let counter = make(1)
  let callback_counter = invoke_counter(make, 4)
  let assert [returned_constructor] = invoke(function_list)
  let returned_external_function = invoke(external_function)
  let returned_function = invoke(function_function)
  let list_function_counter = returned_constructor(5)
  let function_function_counter = returned_function()(6)
  let returned_external_function_counter = returned_external_function(7)
  let values = external_list(2)
  let more_values = make_list(3)
  let assert <<codepoint:utf8_codepoint>> = <<"A":utf8>>
  let codepoints = [codepoint]
  let capture_external = fn() { counter }
  let capture_list = fn() { values }
  let capture_function = fn() { make }
  let capture_codepoints = fn() { codepoints }
  let equal = counter == new_counter(1)

  echo #(new_counter, external_list, external_function) as "functions"
  echo #(
    capture_external,
    capture_list,
    capture_function,
  ) as "captured functions"
  echo #(
    capture_external(),
    capture_list(),
    capture_codepoints(),
  ) as "captures"
  echo #(values, more_values) as "lists"
  echo construct_list_item(8) as "constructed list"
  echo callback_counter as "callback"
  echo #(
    list_function_counter,
    function_function_counter,
    returned_external_function_counter,
  ) as "generic callbacks"
  echo [] as "empty"
  echo equal as "equality"

  counter
}
"#;
        for (entry, failure) in [
            ("exercise()", None),
            (
                "invoke_counter(fn(_) { panic as \"counter callback\" }, 0)",
                Some("counter callback"),
            ),
            (
                "invoke(fn() -> Int { panic as \"generic callback\" })",
                Some("generic callback"),
            ),
        ] {
            let source = format!("{source}\npub fn main() {{ {entry} }}\n");
            let typed = compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new("main", "src/main.gleam", source)],
                )],
                HostProviderSet::with_providers(
                    Vec::<HostModule<ExternalTestProfile>>::new(),
                    [provider()],
                )
                .expect("provider module should be unique"),
            )
            .expect("external runtime source should compile");
            let plan = plan_host_program(typed).expect("external runtime source should plan");
            let mut execution = HostedExecution::try_from_module_plan(plan)
                .expect("external runtime execution should seal");
            let mut echoes = Vec::new();
            let result = crate::execution_fixture::run(
                &mut execution,
                &mut ExternalTestRunState::default(),
                &mut echoes,
            );
            if let Some(message) = failure {
                assert_eq!(result.unwrap_err().to_string(), format!("panic: {message}"));
                assert!(echoes.is_empty());
                continue;
            }
            let returned = result.expect("external runtime source should execute");

            assert_eq!(returned.inspect().to_string(), "Counter(1)");
            assert_eq!(echoes.len(), 9);
            assert_eq!(
                echoes[0].value().inspect().to_string(),
                "#(//fn(a) { ... }, //fn(a) { ... }, //fn() { ... })",
            );
            assert_eq!(
                echoes[1].value().inspect().to_string(),
                "#(//fn() { ... }, //fn() { ... }, //fn() { ... })",
            );
            assert_eq!(
                echoes[2].value().inspect().to_string(),
                "#(Counter(1), [Counter(2), Counter(2)], ['A'])",
            );
            assert_eq!(
                echoes[3].value().inspect().to_string(),
                "#([Counter(2), Counter(2)], [Counter(3), Counter(3)])",
            );
            assert_eq!(echoes[4].value().inspect().to_string(), "[Counter(8)]");
            assert_eq!(echoes[5].value().inspect().to_string(), "Counter(4)");
            assert_eq!(
                echoes[6].value().inspect().to_string(),
                "#(Counter(5), Counter(6), Counter(7))",
            );
            assert_eq!(echoes[7].value().inspect().to_string(), "[]");
            assert_eq!(echoes[8].value(), &Value::Bool(true));
        }
    }
    #[test]
    fn transfer_never_calls_preserve_nested_provider_identity_and_host_caller() {
        use crate::frontend::compile_typed_host_program;
        use crate::host::{
            HostCall, HostFunctionType, HostProfile, HostProvider, HostProviderModule,
            HostProviderSet,
        };
        use std::cell::Cell;

        struct Profile;
        struct Provider;
        impl HostProfile for Profile {
            type RunState = Cell<usize>;
            type ExternalStores = ();
        }
        impl HostProvider<Profile> for Provider {
            type State = Cell<usize>;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        fn stop<'call>(
            mut call: HostCall<'call, Profile, Provider, BigInt>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            callback: HostCallable<'call, HostTypeListEnd, BigInt>,
        ) -> Result<crate::HostCallContinuation<'call, BigInt>, crate::HostCallError> {
            call.state().set(1);
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let value = callback
                        .invoke(&context, |_, _| (), |_, _, value| Ok(value))
                        .await?;
                    assert_eq!(value, BigInt::from(42));
                    Err(crate::HostFailure::new("stopped after callback").into())
                })
            }))
        }
        fn reject<'call>(
            mut call: HostCall<'call, Profile, Provider, BigInt>,
        ) -> Result<HostCallCompletion<'call, BigInt>, crate::HostCallError> {
            call.state().set(2);
            Err(crate::HostFailure::new("native rejected").into())
        }
        let provider = HostProviderModule::new("application", "library")
            .expect("native module")
            .with_scoped_function::<Provider, (), BigInt, _>("reject", reject).expect("reject")
            .with_resumable_function::<Provider, (HostFunctionType<HostTypeListEnd, BigInt>,), BigInt, geam_core::HostTypeListEnd, _>("stop", stop).expect("stop");
        let source = r#"@external(erlang, "native", "reject")
fn reject() -> Int
@external(erlang, "native", "stop")
fn stop(callback: fn() -> Int) -> Int
pub fn run(fails: Bool) {
  case fails {
    True -> stop(reject)
    False -> stop(fn() { echo 42 42 })
  }
}
"#;
        let program = compile_typed_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new("library", "src/library.gleam", source)],
            )],
            HostProviderSet::<Profile>::from_providers([provider]).expect("providers"),
        )
        .expect("source");
        let library = crate::planner::plan_host_library_program(program).expect("plan");
        let template = library
            .functions()
            .iter()
            .find(|function| function.name() == "run")
            .expect("run")
            .signature()
            .id();
        let entry = crate::plan::LibraryEntry::new(
            template,
            crate::plan::LibraryValueType::Int,
            Vec::new(),
            Vec::new(),
        );
        let (plan, entries) =
            crate::plan::execution::HostedProgram::from_library_plan(library, entry, Vec::new())
                .expect("diverging callbacks seal");
        let plan = std::sync::Arc::new(plan);
        for fails in [false, true] {
            let mut state = Cell::new(0);
            let mut output = Vec::new();
            let mut echo = |value: crate::EchoOutput| output.push(value.to_string());
            let mut stores = ();
            let host = crate::execution_fixture::TestHost::default();
            let domain = crate::runtime::execution::Domain::new(
                std::sync::Arc::clone(&plan),
                &host,
                &mut state,
                &mut stores,
                &mut echo,
                std::num::NonZeroUsize::MIN,
            );
            let context = domain.context();
            let mut input = crate::runtime::RetainedValues::empty();
            input.push_evaluated(crate::runtime::EvaluatedValue::Bool(fails));
            let error = host
                .block_on(domain.drive(context.call(
                    *entries.ints[0].function(),
                    crate::runtime::HostCallOrigin::Entry,
                    input,
                )))
                .expect("host cleanup")
                .expect("active entry")
                .expect_err("native function never returns");
            {
                let error = transfer_host_error(&error);
                assert_eq!(error.package(), "application");
                assert_eq!(error.module(), "library");
                assert_eq!(error.function(), if fails { "reject" } else { "stop" });
                assert_eq!(
                    error.failure().message(),
                    if fails {
                        "native rejected"
                    } else {
                        "stopped after callback"
                    }
                );
                assert_eq!(
                    error
                        .location()
                        .caller()
                        .map(|caller| caller.function().as_str()),
                    if fails { Some("stop") } else { None }
                );
                assert_eq!(error.location().line(), if fails { None } else { Some(8) });
            }
            assert_eq!(state.get(), if fails { 2 } else { 1 });
            assert_eq!(
                output,
                if fails {
                    Vec::<&str>::new()
                } else {
                    vec!["src/library.gleam:8\n42"]
                }
            );
        }
    }

    fn transfer_host_error(error: &crate::ExecutionError) -> &crate::HostError {
        match error {
            crate::ExecutionError::Host(error) => error,
            _ => panic!("fixture requires a native host failure"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture requires a native host failure")]
    fn transfer_host_error_rejects_an_invariant_fixture() {
        transfer_host_error(&crate::ExecutionError::Invariant(
            crate::InvariantError::ListIndexOutOfBounds {
                item_type: crate::ValueType::Int,
                index: 0,
                length: 0,
            },
        ));
    }
}
