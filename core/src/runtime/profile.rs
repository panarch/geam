use std::convert::Infallible;

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionGraphProfile, ExecutionHostTarget, ExecutionNeverHostTarget,
    ExecutionProfile, FunctionBodyOwner, RuntimeFunctionFunctionTarget,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{self, RuntimeStateFor};
use crate::runtime::{evaluated, function, graph, host};

pub(in crate::runtime) type RuntimeGraph<Plan> =
    <<Plan as RuntimeExecutionPlan>::Profile as ExecutionProfile>::Graph;

pub(in crate::runtime) trait ExecutableRuntimePlan:
    RuntimeExecutionPlan
{
    type RuntimeHost<'run>: state::RuntimeHostState<State = Self::RunState>
    where
        Self: 'run;

    fn call_host<Body>(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue;

    fn host_parameters<Body>(
        &self,
        target: &ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal]
    where
        Body: ExecutionFunctionBody;

    fn call_host_never(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionNeverHostTarget<Self::Profile>,
        inputs: RetainedValues,
    ) -> ExecutionResult<Infallible>;

    fn host_never_parameters(
        &self,
        target: &ExecutionNeverHostTarget<Self::Profile>,
    ) -> &[ParamLocal];

    fn execute_external_list_instruction(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        environment: &mut graph::BlockEnvironment,
        instruction: &<RuntimeGraph<Self> as ExecutionGraphProfile>::ExternalListInstruction,
        expected: &crate::plan::ValueType,
    ) -> ExecutionResult<()>;

    fn execute_external_function_instruction(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        environment: &mut graph::BlockEnvironment,
        instruction: &<RuntimeGraph<Self> as ExecutionGraphProfile>::ExternalFunctionInstruction,
    ) -> ExecutionResult<()>;

    fn run_function_return(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        function: <RuntimeGraph<Self> as ExecutionGraphProfile>::RuntimeFunctionFunctionId,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<evaluated::EvaluatedFunctionValue>;
}

impl ExecutableRuntimePlan for ExecutionPlan {
    type RuntimeHost<'run> = ();

    fn call_host<Body>(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        _inputs: RetainedValues,
    ) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue,
    {
        match *target {}
    }

    fn host_parameters<Body>(
        &self,
        target: &ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal]
    where
        Body: ExecutionFunctionBody,
    {
        match *target {}
    }

    fn call_host_never(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _origin: HostCallOrigin,
        target: &ExecutionNeverHostTarget<Self::Profile>,
        _inputs: RetainedValues,
    ) -> ExecutionResult<Infallible> {
        match *target {}
    }

    fn host_never_parameters(
        &self,
        target: &ExecutionNeverHostTarget<Self::Profile>,
    ) -> &[ParamLocal] {
        match *target {}
    }

    fn execute_external_list_instruction(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _environment: &mut graph::BlockEnvironment,
        instruction: &Infallible,
        _expected: &crate::plan::ValueType,
    ) -> ExecutionResult<()> {
        match *instruction {}
    }

    fn execute_external_function_instruction(
        &self,
        _state: &mut RuntimeStateFor<'_, Self>,
        _environment: &mut graph::BlockEnvironment,
        instruction: &Infallible,
    ) -> ExecutionResult<()> {
        match *instruction {}
    }

    fn run_function_return(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        function: <RuntimeGraph<Self> as ExecutionGraphProfile>::RuntimeFunctionFunctionId,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<evaluated::EvaluatedFunctionValue> {
        function::run_core_function(self, state, function, origin, inputs)
    }
}

impl<Profile: crate::HostProfile> ExecutableRuntimePlan
    for crate::plan::execution::HostedExecution<Profile>
{
    type RuntimeHost<'run>
        = &'run mut Profile::RunState
    where
        Self: 'run;

    fn call_host<Body>(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionHostTarget<Self::Profile, Body>,
        inputs: RetainedValues,
    ) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
    where
        Body: ExecutionFunctionBody,
        Body::Return: graph::GraphValue,
    {
        match target {
            crate::plan::execution::host::HostedFunctionTarget::Value(target) => {
                host::invoke_value(self, state, origin, target, inputs)
            }
            crate::plan::execution::host::HostedFunctionTarget::Never(target) => {
                host::invoke_never(self, state, origin, *target, inputs).map(|never| match never {})
            }
        }
    }

    fn host_parameters<Body>(
        &self,
        target: &ExecutionHostTarget<Self::Profile, Body>,
    ) -> &[ParamLocal]
    where
        Body: ExecutionFunctionBody,
    {
        match target {
            crate::plan::execution::host::HostedFunctionTarget::Value(target) => {
                self.host_value_function(target).parameters()
            }
            crate::plan::execution::host::HostedFunctionTarget::Never(target) => {
                self.host_never_function(*target).parameters()
            }
        }
    }

    fn call_host_never(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        origin: HostCallOrigin,
        target: &ExecutionNeverHostTarget<Self::Profile>,
        inputs: RetainedValues,
    ) -> ExecutionResult<Infallible> {
        host::invoke_never(self, state, origin, *target, inputs)
    }

    fn host_never_parameters(
        &self,
        target: &ExecutionNeverHostTarget<Self::Profile>,
    ) -> &[ParamLocal] {
        self.host_never_function(*target).parameters()
    }

    fn execute_external_list_instruction(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        environment: &mut graph::BlockEnvironment,
        instruction: &crate::plan::execution::graph::ExternalListInstruction,
        expected: &crate::plan::ValueType,
    ) -> ExecutionResult<()> {
        graph::execute_external_list_instruction(self, state, environment, instruction, expected)
    }

    fn execute_external_function_instruction(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        environment: &mut graph::BlockEnvironment,
        instruction: &crate::plan::execution::graph::ExternalFunctionInstruction,
    ) -> ExecutionResult<()> {
        graph::execute_external_function_instruction(self, state, environment, instruction)
    }

    fn run_function_return(
        &self,
        state: &mut RuntimeStateFor<'_, Self>,
        function: <RuntimeGraph<Self> as ExecutionGraphProfile>::RuntimeFunctionFunctionId,
        origin: HostCallOrigin,
        inputs: RetainedValues,
    ) -> ExecutionResult<evaluated::EvaluatedFunctionValue> {
        match function {
            RuntimeFunctionFunctionTarget::Core(function) => {
                function::run_core_function(self, state, function, origin, inputs)
            }
            RuntimeFunctionFunctionTarget::External(function) => {
                function::run_external_function_function(self, state, function, origin, inputs)
            }
        }
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
        let retained_hash = |_: &crate::runtime::StoredRuntimeValue| 0;
        let hashing = crate::host::HostExternalHashing::new(&retained_hash);
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
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeHostCounter,
            >,
            function: HostCallable<'call, RuntimeIntArguments, RuntimeHostCounter>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, RuntimeHostCounter>, HostCallError> {
            let counter = call
                .invoke(function, (value, ()))
                .expect("counter callback should succeed");
            Ok(call.return_value(counter))
        }

        fn invoke<'call>(
            mut call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeGenericValue,
            >,
            function: HostCallable<'call, HostTypeListEnd, RuntimeGenericValue>,
        ) -> Result<HostCallCompletion<'call, RuntimeGenericValue>, HostCallError> {
            let value = call
                .invoke(function, ())
                .expect("generic callback should succeed");
            Ok(call.return_value(value))
        }

        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
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
            .with_scoped_function::<
                RuntimeCounterProvider,
                (RuntimeCounterCallable, BigInt),
                RuntimeHostCounter,
                _,
            >(
                "invoke_counter",
                invoke_counter,
            )
            .expect("external callback should be valid")
            .with_scoped_function::<
                RuntimeCounterProvider,
                (RuntimeGenericCallable,),
                RuntimeGenericValue,
                _,
            >("invoke", invoke)
            .expect("generic callback should be valid");
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

pub fn main() {
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
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("external runtime source should compile");
        let plan = plan_host_program(typed).expect("external runtime source should plan");
        let execution = HostedExecution::try_from_module_plan(plan)
            .expect("external runtime execution should seal");
        let mut echoes = Vec::new();
        let returned = execution
            .run_main(&mut ExternalTestRunState::default(), &mut echoes)
            .expect("external runtime source should execute");

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
