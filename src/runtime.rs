mod constant;
mod echo;
mod error;
mod evaluated;
mod function;
mod graph;
mod host;
mod materialize;
mod state;
mod value;

pub use echo::{EchoLocation, EchoOutput, EchoSink};
pub use error::{
    BitArraySegmentPanicReason, ExecutionError, HostError, HostLocation, HostOrigin,
    InvariantError, Panic, PanicDetails, PanicKind, PanicMessage,
};
pub(in crate::runtime) use evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCapture,
    EvaluatedCustomFunction, EvaluatedCustomValue, EvaluatedExternalFunction,
    EvaluatedExternalValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValueKind, EvaluatedGenericFunction, EvaluatedIntFunction,
    EvaluatedListFunction, EvaluatedNeverFunction, EvaluatedNilFunction, EvaluatedStringFunction,
    EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
#[cfg(test)]
pub(in crate::runtime) use evaluated::{EvaluatedFunctionValue, EvaluatedListCapture};
pub(crate) use value::{
    BitArrayFunctionValue, BoolFunctionValue, CaptureListValue, CaptureValue, CustomFunctionValue,
    CustomFunctionValueTarget, ExternalFunctionValue, FloatFunctionValue, FunctionFunctionValue,
    FunctionValueKind, GenericFunctionValue, IntFunctionValue, ListFunctionValue,
    NeverFunctionValue, NilFunctionValue, StringFunctionValue, TupleFunctionValue,
    UtfCodepointFunctionValue,
};
pub use value::{
    BitArrayValue, BitArrayValueLengthError, CustomFieldValue, CustomValue, ExternalValue,
    ExternalValueIdentity, FunctionValue, ListValue, ListValueItemTypeMismatch, Value,
    ValueInspection,
};

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::function::{
    ExecutionFunctionBody, ExecutionGraphProfile, ExecutionHostTarget, ExecutionNeverHostTarget,
    ExecutionProfile, FunctionBodyOwner, ProfiledCoreRuntimeFunctionId, ProfiledRuntimeFunctionId,
    RuntimeFunctionFunctionTarget,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::runtime::RuntimeExecutionPlan;
use crate::runtime::error::{ExecutionResult, HostCallOrigin};
use crate::runtime::graph::RetainedValues;
use crate::runtime::state::{RuntimeState, RuntimeStateFor};
use std::convert::Infallible;

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
                invoke_host_value(self, state, origin, target, inputs)
            }
            crate::plan::execution::host::HostedFunctionTarget::Never(target) => {
                invoke_host_never(self, state, origin, *target, inputs).map(|never| match never {})
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
        invoke_host_never(self, state, origin, *target, inputs)
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

fn invoke_host_value<'run, Profile, Body>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'run, crate::plan::execution::HostedExecution<Profile>>,
    origin: HostCallOrigin,
    target: &crate::plan::execution::host::HostFunctionId<Body>,
    inputs: RetainedValues,
) -> ExecutionResult<<<Body as FunctionBodyOwner>::Return as graph::GraphValue>::Evaluated>
where
    Profile: crate::HostProfile,
    Body: ExecutionFunctionBody,
    Body::Return: graph::GraphValue,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    let function = plan.host_value_function(target);
    let mut call = host::RuntimeHostCall::new(plan, state, function, inputs);
    match function.implementation().call(&mut call) {
        Ok(returned) => Ok(call.finish(returned, target.return_())),
        Err(error) => {
            drop(call);
            state.values_mut().drain_releases();
            Err(host_call_error(plan, origin, function.metadata(), error))
        }
    }
}

fn invoke_host_never<'run, Profile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'run, crate::plan::execution::HostedExecution<Profile>>,
    origin: HostCallOrigin,
    target: crate::plan::execution::host::HostNeverFunctionId,
    inputs: RetainedValues,
) -> ExecutionResult<std::convert::Infallible>
where
    Profile: crate::HostProfile,
    crate::plan::execution::HostedExecution<Profile>: 'run,
{
    let function = plan.host_never_function(target);
    let mut call = host::RuntimeHostCall::new(plan, state, function, inputs);
    match function.implementation().call(&mut call) {
        Ok(never) => match never {},
        Err(error) => {
            drop(call);
            state.values_mut().drain_releases();
            Err(host_call_error(plan, origin, function.metadata(), error))
        }
    }
}

fn host_call_error<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    origin: HostCallOrigin,
    function: &crate::plan::execution::host::HostedFunctionMetadata,
    error: crate::host::HostCallError,
) -> ExecutionError {
    match error.into_kind() {
        crate::host::HostCallErrorKind::Nested(error) => error,
        crate::host::HostCallErrorKind::Failure(failure) => {
            match origin.into_source_site(function.site()) {
                Ok(site) => ExecutionError::from_host_call(
                    function,
                    site.clone(),
                    plan.source_context_for(site.module()),
                    failure,
                ),
                Err(caller) => ExecutionError::from_host_origin(function, caller, failure),
            }
        }
    }
}

pub fn run_main(plan: &ExecutionPlan, echo: &mut dyn EchoSink) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::new(echo);
    let function = match RuntimeExecutionPlan::main_runtime(plan) {
        ProfiledRuntimeFunctionId::Core(function) => function,
        ProfiledRuntimeFunctionId::External(function) => match function {},
    };
    let value = run_core_program(plan, &mut state, function, RetainedValues::empty())?;
    finish_program(plan, &mut state, value)
}

pub(crate) fn run_hosted_main<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    host: &mut Profile::RunState,
    echo: &mut dyn EchoSink,
) -> Result<Value, ExecutionError> {
    let mut state = RuntimeState::with_host(echo, host);
    run_hosted_program_inner(plan, &mut state)
}

#[cfg(test)]
fn run_hosted_program<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'_, crate::plan::execution::HostedExecution<Profile>>,
) -> Result<Value, ExecutionError> {
    run_hosted_program_inner(plan, state)
}

fn run_hosted_program_inner<Profile: crate::HostProfile>(
    plan: &crate::plan::execution::HostedExecution<Profile>,
    state: &mut RuntimeStateFor<'_, crate::plan::execution::HostedExecution<Profile>>,
) -> Result<Value, ExecutionError> {
    let inputs = RetainedValues::empty();
    let value = match plan.main_runtime() {
        ProfiledRuntimeFunctionId::Core(function) => {
            run_core_program(plan, state, function, inputs)
        }
        ProfiledRuntimeFunctionId::External(function) => {
            function::run_external(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::External)
        }
    }?;
    finish_program(plan, state, value)
}

fn run_core_program<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    function: ProfiledCoreRuntimeFunctionId<RuntimeGraph<Plan>>,
    inputs: RetainedValues,
) -> ExecutionResult<EvaluatedValue>
where
    Plan: ExecutableRuntimePlan,
{
    match function {
        ProfiledCoreRuntimeFunctionId::Never(function) => {
            function::run_never(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(|never| match never {})
        }
        ProfiledCoreRuntimeFunctionId::Int(function) => {
            function::run_int(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Int)
        }
        ProfiledCoreRuntimeFunctionId::Float(function) => {
            function::run_float(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Float)
        }
        ProfiledCoreRuntimeFunctionId::String(function) => {
            function::run_string(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::String)
        }
        ProfiledCoreRuntimeFunctionId::BitArray(function) => {
            function::run_bit_array(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::BitArray)
        }
        ProfiledCoreRuntimeFunctionId::UtfCodepoint(function) => {
            function::run_utf_codepoint(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::UtfCodepoint)
        }
        ProfiledCoreRuntimeFunctionId::Custom(function) => {
            function::run_custom(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Custom)
        }
        ProfiledCoreRuntimeFunctionId::Bool(function) => {
            function::run_bool(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Bool)
        }
        ProfiledCoreRuntimeFunctionId::Nil(function) => {
            function::run_nil(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(|()| EvaluatedValue::Nil)
        }
        ProfiledCoreRuntimeFunctionId::Tuple { id, .. } => {
            function::run_tuple(plan, state, id, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::Tuple)
        }
        ProfiledCoreRuntimeFunctionId::List(function) => {
            let function = <RuntimeGraph<Plan> as ExecutionGraphProfile>::list_function(&function);
            function::run_list(plan, state, function, error::HostCallOrigin::Entry, inputs)
                .map(EvaluatedValue::from)
        }
        ProfiledCoreRuntimeFunctionId::Function { id, .. } => plan
            .run_function_return(state, id, error::HostCallOrigin::Entry, inputs)
            .map(EvaluatedValue::Function),
    }
}

fn finish_program<Plan>(
    plan: &Plan,
    state: &mut RuntimeStateFor<'_, Plan>,
    value: EvaluatedValue,
) -> Result<Value, ExecutionError>
where
    Plan: ExecutableRuntimePlan,
{
    state.values_mut().drain_releases();
    Ok(materialize::value(
        plan.value_metadata(),
        state.values(),
        value,
    ))
}

#[cfg(test)]
fn run_src(src: &str) -> Value {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    let plan = crate::ExecutionPlan::from_module_plan(module_plan);
    run_main(&plan, &mut Vec::new()).expect("source should run")
}

#[cfg(test)]
fn run_src_error(src: &str) -> ExecutionError {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    let plan = crate::ExecutionPlan::from_module_plan(module_plan);
    run_main(&plan, &mut Vec::new()).expect_err("source should fail at runtime")
}

#[cfg(test)]
fn plan_src(src: &str) -> crate::ExecutionPlan {
    let module =
        crate::compile_typed_module("main", "main.gleam", src).expect("source should compile");
    let module_plan = crate::plan_module(module).expect("source should plan");
    crate::ExecutionPlan::from_module_plan(module_plan)
}

#[cfg(test)]
fn int(value: i64) -> Value {
    Value::Int(num_bigint::BigInt::from(value))
}

#[cfg(test)]
mod tests {
    use super::{BitArrayValue, ListValue, Value, int, run_src};
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, ExternalTestStores, HostExternalSchema,
        HostExternalStorage, HostExternalStore,
    };
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostExternal, HostExternalType,
        HostFailure, HostFunctionType, HostListType, HostModule, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeList, HostTypeListEnd, HostTypeParameter, HostedExecution,
        ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;
    use std::convert::Infallible;

    struct RuntimeCounterSchema;
    struct RuntimeCounterProvider;
    type RuntimeHostCounter = HostExternalType<RuntimeCounterSchema>;
    type RuntimeIntArguments = HostTypeList<BigInt, HostTypeListEnd>;
    type RuntimeCounterCallable = HostFunctionType<RuntimeIntArguments, RuntimeHostCounter>;
    type RuntimeGenericValue = HostTypeParameter<0>;
    type RuntimeGenericCallable = HostFunctionType<HostTypeListEnd, RuntimeGenericValue>;

    impl HostExternalSchema for RuntimeCounterSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Counter";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<RuntimeCounterSchema> for ExternalTestProfile {
        type Payload = BigInt;

        fn store(stores: &ExternalTestStores) -> &HostExternalStore<Self::Payload> {
            &stores.integers
        }

        fn equal(left: &Self::Payload, right: &Self::Payload) -> bool {
            left == right
        }

        fn inspect(value: &Self::Payload) -> EcoString {
            format!("Counter({value})").into()
        }
    }

    impl HostProvider<ExternalTestProfile> for RuntimeCounterProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    #[test]
    fn run_main() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  1
}
"#,
            ),
            int(1),
        );
    }

    #[test]
    fn run_main_materializes_utf_codepoint_and_nil_returns() {
        assert_eq!(
            run_src("pub fn main() { let assert <<value:utf8_codepoint>> = <<65>> value }"),
            Value::UtfCodepoint('A'),
        );
        assert_eq!(run_src("pub fn main() { Nil }"), Value::Nil);
    }

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
            .with_external_type::<RuntimeCounterSchema>()
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
        assert_eq!(echoes.len(), 8);
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
        assert_eq!(echoes[4].value().inspect().to_string(), "Counter(4)");
        assert_eq!(
            echoes[5].value().inspect().to_string(),
            "#(Counter(5), Counter(6), Counter(7))",
        );
        assert_eq!(echoes[6].value().inspect().to_string(), "[]");
        assert_eq!(echoes[7].value(), &Value::Bool(true));
    }

    #[test]
    fn hosted_runtime_routes_diverging_external_return_families() {
        fn stop_counter<'call>(
            _call: HostCall<'call, ExternalTestProfile, RuntimeCounterProvider, RuntimeHostCounter>,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("counter stopped").into())
        }

        fn stop_list<'call>(
            _call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                HostListType<RuntimeHostCounter>,
            >,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("list stopped").into())
        }

        fn stop_function<'call>(
            _call: HostCall<
                'call,
                ExternalTestProfile,
                RuntimeCounterProvider,
                RuntimeCounterCallable,
            >,
        ) -> Result<Infallible, HostCallError> {
            Err(HostFailure::new("function stopped").into())
        }

        enum ReturnFamily {
            Value,
            List,
            Function,
        }

        let cases = [
            (
                ReturnFamily::Value,
                r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop_counter")
fn stop_counter() -> Counter

fn forward() -> Counter {
  stop_counter()
}

pub fn main() {
  let _ = forward()
  1
}
"#,
                "host function application::main.stop_counter failed: counter stopped",
            ),
            (
                ReturnFamily::List,
                r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop_list")
fn stop_list() -> List(Counter)

fn forward() -> List(Counter) {
  stop_list()
}

pub fn main() {
  let _ = forward()
  1
}
"#,
                "host function application::main.stop_list failed: list stopped",
            ),
            (
                ReturnFamily::Function,
                r#"
@external(erlang, "host", "Counter")
pub type Counter

@external(erlang, "host", "stop_function")
fn stop_function() -> fn(Int) -> Counter

fn forward() -> fn(Int) -> Counter {
  stop_function()
}

pub fn main() {
  let _ = forward()
  1
}
"#,
                "host function application::main.stop_function failed: function stopped",
            ),
        ];

        for (family, source, expected_error) in cases {
            let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
                .expect("provider module should be valid");
            let provider = match family {
                ReturnFamily::Value => provider
                    .with_external_type::<RuntimeCounterSchema>()
                    .expect("external type should be valid")
                    .with_scoped_diverging_function::<
                        RuntimeCounterProvider,
                        (),
                        RuntimeHostCounter,
                        _,
                    >(
                        "stop_counter",
                        stop_counter,
                    )
                    .expect("external value target should be valid"),
                ReturnFamily::List => provider
                    .with_external_type::<RuntimeCounterSchema>()
                    .expect("external type should be valid")
                    .with_scoped_diverging_function::<
                        RuntimeCounterProvider,
                        (),
                        HostListType<RuntimeHostCounter>,
                        _,
                    >("stop_list", stop_list)
                    .expect("external list target should be valid"),
                ReturnFamily::Function => provider
                    .with_external_type::<RuntimeCounterSchema>()
                    .expect("external type should be valid")
                    .with_scoped_diverging_function::<
                        RuntimeCounterProvider,
                        (),
                        RuntimeCounterCallable,
                        _,
                    >(
                        "stop_function",
                        stop_function,
                    )
                    .expect("external function target should be valid"),
            };
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
            .expect("diverging external source should compile");
            let plan = plan_host_program(typed).expect("diverging external source should plan");
            let execution = HostedExecution::try_from_module_plan(plan)
                .expect("diverging external execution should seal");
            let error = execution
                .run_main(&mut ExternalTestRunState::default(), &mut Vec::new())
                .expect_err("diverging external target should fail");

            assert_eq!(error.to_string(), expected_error);
        }
    }

    #[test]
    fn source_constants_preserve_runtime_values_and_function_identity() {
        let source = r#"
pub type Boxed(value) { Boxed(value) }

const int = 1
const float = 1.5
const string = "geam"
const bit_array = <<1>>
const bool = True
const nil = Nil
const tuple = #(1, "one")
const list = [1, 2]
const empty = []
const other_empty = []
const nested = [[]]
const boxed = Boxed(1)
const function = identity
const other_function = identity

fn identity(value) { value }

pub fn main() {
  #(
    int,
    float,
    string,
    bit_array,
    bool,
    nil,
    tuple,
    list,
    empty == [],
    empty == other_empty,
    nested == [[]],
    boxed == Boxed(1),
    function == function,
    function == other_function,
  )
}
"#;

        assert_eq!(
            run_src(source),
            Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Float(1.5),
                Value::String("geam".into()),
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::Bool(true),
                Value::Nil,
                Value::Tuple(vec![Value::Int(1.into()), Value::String("one".into())]),
                Value::List(ListValue::int(vec![1.into(), 2.into()])),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
                Value::Bool(true),
            ]),
        );
    }

    #[test]
    fn constants_referenced_only_by_unreachable_functions_are_not_evaluated() {
        let source = r#"
const failing = <<<<1>>:bits-size(16)>>

fn unused() {
  failing
}

pub fn main() {
  1
}
"#;

        assert_eq!(run_src(source), Value::Int(1.into()));
    }

    #[test]
    fn constants_are_evaluated_only_when_their_reference_is_evaluated() {
        let source = r#"
const failing = <<<<1>>:bits-size(16)>>

pub fn main() {
  case False {
    True -> failing
    False -> <<>>
  }
}
"#;

        assert_eq!(
            run_src(source),
            Value::BitArray(BitArrayValue::from_bytes(Vec::new())),
        );
    }

    #[test]
    fn function_constants_preserve_reference_and_instance_identity() {
        let source = r#"
pub type Boxed(value) { Boxed(value) }

const constructor = Boxed
const function = identity

fn identity(value) { value }

pub fn main() {
  #(
    constructor == constructor,
    Boxed == Boxed,
    function == function,
    identity == identity,
  )
}
"#;

        assert_eq!(
            run_src(source),
            Value::Tuple(vec![
                Value::Bool(false),
                Value::Bool(false),
                Value::Bool(true),
                Value::Bool(true),
            ]),
        );
    }
}
