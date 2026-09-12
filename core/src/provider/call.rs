use super::{
    Callback, ProviderCallbackCodec, ProviderExternalCodec, ProviderExternalView,
    ProviderOwnedCallbackContext, ProviderOwnedStoredInput, ProviderStoredInput,
    ProviderStoredOutput, ProviderStoredOwner, ProviderValueContext, ProviderValueForms, Stored,
    Value,
};
use crate::host::{
    HostExecutionContext, HostExecutionError, HostFutureContext, HostTypeListEnd, HostTypeSequence,
};
use crate::provider::advanced::{
    NativeValue, ProviderDynamicInput, ProviderDynamicValue, Retained, StoredDynamic,
};
use crate::{HostCall, HostCallError, HostListType, HostProfile, HostProvider, HostType};
use ecow::EcoString;
use std::marker::PhantomData;

/// Access to provider-owned state and active-call capabilities.
///
/// Provider functions receive this type through `#[geam::call]`. The macro
/// replaces the placeholder context with one statically tied to the registered
/// provider function.
pub struct Call<State, Context = ProviderCallPlaceholder> {
    context: Context,
    state: PhantomData<fn() -> State>,
}

/// A provider failure that stops the active source execution.
pub type HostResult<Value, Error = HostCallError> = Result<Value, Error>;

#[doc(hidden)]
pub struct ProviderCallPlaceholder;

#[doc(hidden)]
pub struct ProviderSharedCall<'state, State> {
    state: &'state State,
}

/// Active immediate call context for a transferable provider composition.
#[doc(hidden)]
pub struct ProviderActiveCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call: HostCall<'call, Profile, Provider, Return>,
}

/// Request capability shared by owned native operations.
#[doc(hidden)]
pub struct ProviderExecutionCall<'run, Profile, Provider, Observation = ()>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    execution: HostExecutionContext<'run, Profile, Provider, HostTypeListEnd>,
    observation: Observation,
}

#[doc(hidden)]
pub struct ProviderWorkObservation {
    dependencies: crate::runtime::work::Dependencies<crate::runtime::work::execution::Completion>,
}

#[doc(hidden)]
pub type ProviderFutureCall<'run, Profile, Provider> =
    ProviderExecutionCall<'run, Profile, Provider, ProviderWorkObservation>;

impl<'state, State> Call<State, ProviderSharedCall<'state, State>> {
    pub fn state(&self) -> &State {
        self.context.state
    }

    #[doc(hidden)]
    pub fn from_shared_state(state: &'state State) -> Self {
        Self {
            context: ProviderSharedCall { state },
            state: PhantomData,
        }
    }
}

impl<'call, Profile, Provider, Return>
    Call<Provider::State, ProviderActiveCall<'call, Profile, Provider, Return>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    pub fn state(&mut self) -> &Provider::State {
        &*self.context.call.state()
    }

    pub fn state_mut(&mut self) -> &mut Provider::State {
        self.context.call.state()
    }

    pub fn equal<Type, Host>(
        &self,
        left: &Value<Type, crate::provider::ProviderValueContext<Host>>,
        right: &Value<Type, crate::provider::ProviderValueContext<Host>>,
    ) -> bool
    where
        Host: HostType,
    {
        self.context
            .call
            .stored_equal(left.stored(), right.stored())
    }

    pub fn source_hash<Type, Host>(
        &self,
        value: &Value<Type, crate::provider::ProviderValueContext<Host>>,
    ) -> u64
    where
        Host: HostType,
    {
        self.context.call.stored_source_hash(value.stored())
    }

    /// Compares declared native representations without changing source types.
    pub fn native_equal(&self, left: &NativeValue, right: &NativeValue) -> bool {
        self.context.call.native_equal(left, right)
    }

    /// Hashes a declared native representation consistently with native equality.
    pub fn native_hash(&self, value: &NativeValue) -> u64 {
        self.context.call.native_hash(value)
    }

    /// Hashes the native representation of a retained source value.
    pub fn native_source_hash<Type, Host>(
        &self,
        value: &Value<Type, ProviderValueContext<Host>>,
    ) -> u64
    where
        Host: HostType,
    {
        self.native_hash(&NativeValue::from_stored(value.stored().clone_retained()))
    }

    /// Views a received List as a native tuple without decoding its elements.
    pub fn native_tuple<Item, HostItem, Decoder>(
        &mut self,
        values: super::List<Item, super::ProviderListContext<HostItem, Decoder>>,
    ) -> NativeValue
    where
        HostItem: HostType,
        Decoder: super::ProviderListItemDecoder<Item>,
    {
        let context = values.__geam_into_context();
        let values = self
            .context
            .call
            .restore_list_value::<HostItem>(context.retained());
        self.context.call.native_tuple(values)
    }

    pub fn inspect<Type, Host>(
        &self,
        value: &Value<Type, crate::provider::ProviderValueContext<Host>>,
    ) -> EcoString
    where
        Host: HostType,
    {
        self.context.call.stored_inspect(value.stored())
    }

    pub fn list_len<ListType, ItemHost>(
        &self,
        value: &Value<ListType, crate::provider::ProviderValueContext<HostListType<ItemHost>>>,
    ) -> usize
    where
        ItemHost: HostType,
    {
        self.context.call.stored_list_len(value.stored())
    }

    pub fn list_get<ListType, Item, ItemHost>(
        &mut self,
        value: &Value<ListType, crate::provider::ProviderValueContext<HostListType<ItemHost>>>,
        index: usize,
    ) -> Option<Value<Item, crate::provider::ProviderValueContext<ItemHost>>>
    where
        ItemHost: HostType,
    {
        self.context
            .call
            .stored_list_item(value.stored(), index)
            .map(Value::from_stored)
    }

    /// Retains one transferable generic value for its generated payload.
    pub fn store<Type, Host, Owner, Index>(
        &mut self,
        value: Value<Type, ProviderValueContext<Host>>,
    ) -> Stored<Type, ProviderStoredOutput<Owner, Index, Host>>
    where
        Host: HostType,
        Owner: ProviderStoredOwner,
    {
        Stored::from_output(Retained::from_runtime_value(value.into_stored()))
    }

    /// Restores one generic value selected from a transferable external input.
    pub fn restore<Type, Host, Owner, Index>(
        &mut self,
        value: Stored<Type, ProviderStoredInput<'_, Owner, Index, Host>>,
    ) -> Value<Type, ProviderValueContext<Host>>
    where
        Host: HostType,
        Owner: ProviderStoredOwner,
    {
        Value::from_stored(value.retained().stored().clone_retained())
    }

    /// Reads the payload of a statically known transferable external value.
    #[doc(hidden)]
    pub fn external_payload<Type>(
        &mut self,
        value: Value<Type, ProviderValueContext<Type::Host>>,
    ) -> ProviderExternalView<Type::Output>
    where
        Type: ProviderValueForms,
        Type::Output: ProviderExternalCodec<Profile>,
    {
        let value = value.into_host(&mut self.context.call);
        Type::Output::immediate_input(&self.context.call, value)
    }

    /// Retains one transferable value with its exact specialized type.
    pub fn store_dynamic<Value, Owner>(&mut self, value: Value) -> StoredDynamic<Owner>
    where
        Value: ProviderDynamicValue<'call, Profile, Provider, Return>,
        Owner: ProviderStoredOwner,
    {
        value.into_stored::<Owner>(&mut self.context.call)
    }

    /// Restores an existential transferable value only at its exact type.
    pub fn restore_dynamic<Type, Owner>(
        &mut self,
        value: &StoredDynamic<Owner>,
    ) -> Option<Type::View>
    where
        Type: ProviderDynamicInput<Profile, Provider, Return>,
        Owner: ProviderStoredOwner,
    {
        if !self
            .context
            .call
            .stored_has_type::<Type::Host>(value.stored())
        {
            return None;
        }
        let value = self
            .context
            .call
            .restore_value::<Type::Host>(value.stored());
        Some(Type::from_host(&mut self.context.call, value))
    }

    /// Restores an exact source value carried by a declared native view.
    ///
    /// This does not construct a different source type from the native data.
    pub fn restore_native<Type>(&mut self, value: &NativeValue) -> Option<Type::View>
    where
        Type: ProviderDynamicInput<Profile, Provider, Return>,
    {
        let value = value.find_source(|value| {
            self.context
                .call
                .native_has_type::<Type::Host>(value)
                .then(|| value.clone_retained())
        })?;
        let value = self.context.call.restore_value::<Type::Host>(&value);
        Some(Type::from_host(&mut self.context.call, value))
    }

    /// Restores an existential value with a transferable type witness.
    pub fn restore_dynamic_value<Type, Host, Owner>(
        &mut self,
        value: &StoredDynamic<Owner>,
        _type_witness: &Value<Type, ProviderValueContext<Host>>,
    ) -> Option<Value<Type, ProviderValueContext<Host>>>
    where
        Host: HostType,
        Owner: ProviderStoredOwner,
    {
        self.context
            .call
            .stored_has_type::<Host>(value.stored())
            .then(|| Value::from_stored(value.stored().clone_retained()))
    }

    #[doc(hidden)]
    pub fn from_host_call(call: HostCall<'call, Profile, Provider, Return>) -> Self {
        Self {
            context: ProviderActiveCall { call },
            state: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn into_host_call(self) -> HostCall<'call, Profile, Provider, Return> {
        self.context.call
    }
}

impl<'run, Profile, Provider, Observation>
    Call<Provider::State, ProviderExecutionCall<'run, Profile, Provider, Observation>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    /// Runs a bounded value operation through the original typed call context.
    /// Its borrowed views cannot cross the next await point.
    pub fn with_call<'request, Operation, Output>(
        &'request self,
        operation: Operation,
    ) -> impl std::future::Future<Output = Result<Output, HostExecutionError>> + Send + 'request
    where
        Operation: for<'call> FnOnce(
                &mut Call<Provider::State, ProviderActiveCall<'call, Profile, Provider, ()>>,
            ) -> Output
            + Send
            + 'static,
        Output: Send + 'static,
    {
        self.context
            .execution
            .with_call(move |call| operation(&mut Call::from_host_call(call)))
    }

    /// Retains one owned generic value for an external payload returned after
    /// this async provider call completes.
    pub fn store<Type, Host, Owner, Index>(
        &mut self,
        value: Value<Type, ProviderValueContext<Host>>,
    ) -> Stored<Type, ProviderStoredOutput<Owner, Index, Host>>
    where
        Host: HostType,
        Owner: ProviderStoredOwner,
    {
        Stored::from_output(Retained::from_runtime_value(value.into_stored()))
    }

    /// Restores one retained generic value owned by this async invocation.
    pub fn restore<Type, Host, Owner, Index>(
        &mut self,
        value: Stored<Type, ProviderOwnedStoredInput<Owner, Index, Host>>,
    ) -> Value<Type, ProviderValueContext<Host>>
    where
        Host: HostType,
        Owner: ProviderStoredOwner,
    {
        Value::from_stored(value.into_async_stored().stored().clone_retained())
    }

    /// Runs one bounded operation against the provider's original execution state.
    pub async fn with_state<Operation, Output>(
        &mut self,
        operation: Operation,
    ) -> Result<Output, HostExecutionError>
    where
        Operation: FnOnce(&mut Provider::State) -> Output + Send + 'static,
        Output: Send + 'static,
    {
        self.context.execution.with_state(operation).await
    }

    /// Invokes a retained callback on its original execution, without implicitly driving its result.
    pub async fn invoke<Signature, Codec>(
        &mut self,
        callback: &Callback<Signature, ProviderOwnedCallbackContext<Profile, Provider, Codec>>,
        arguments: Codec::Arguments,
    ) -> Result<Codec::Returned, HostExecutionError>
    where
        Codec: ProviderCallbackCodec<Profile, Provider, ()> + 'static,
        Codec::Arguments: Send + 'static,
        Codec::Returned: Send + 'static,
    {
        callback
            .invoke_owned(&self.context.execution, arguments)
            .await
    }
}

impl<'run, Profile, Provider> Call<Provider::State, ProviderExecutionCall<'run, Profile, Provider>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    #[doc(hidden)]
    pub fn from_execution_context<Constructions: HostTypeSequence>(
        execution: HostExecutionContext<'run, Profile, Provider, Constructions>,
    ) -> Self {
        Self {
            context: ProviderExecutionCall {
                execution: execution.without_constructions(),
                observation: (),
            },
            state: PhantomData,
        }
    }
}

impl<'work, Profile, Provider> Call<Provider::State, ProviderFutureCall<'work, Profile, Provider>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    /// Explicitly observes a Future returned by Gleam, sharing its original work.
    pub async fn observe<Value, Host, Output>(
        &mut self,
        work: &super::Future<
            Value,
            super::ProviderFutureValueContext<Profile, Provider, Host, Output>,
        >,
    ) -> Result<Output, HostExecutionError>
    where
        Host: HostType,
        Output: Send + 'static,
    {
        work.observe(&self.context.observation.dependencies).await
    }

    #[doc(hidden)]
    pub fn from_future_context<Constructions: HostTypeSequence>(
        call: HostFutureContext<'work, Profile, Provider, Constructions>,
    ) -> Self {
        let (execution, dependencies) = call.into_parts();
        Self {
            context: ProviderExecutionCall {
                execution: execution.without_constructions(),
                observation: ProviderWorkObservation { dependencies },
            },
            state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Call, HostResult};
    use crate::host::CallArguments;
    use crate::host::HostCallErrorKind;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{HostCallable, HostTypeList, HostTypeListEnd, HostTypeParameter};
    use crate::provider::{
        Callback, ProviderCallbackCodec, ProviderConstructions, ProviderNoConstructions,
        ProviderOwnedCallbackContext, ProviderValueContext, Value,
    };
    use crate::{HostCall, HostFailure, HostProvider};
    use num_bigint::BigInt;

    struct Provider;

    impl HostProvider<TestHostProfile> for Provider {
        type State = TestRunState;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            state
        }
    }

    struct IntCallbackCodec;

    impl ProviderCallbackCodec<TestHostProfile, Provider, ()> for IntCallbackCodec {
        type HostArguments = HostTypeList<BigInt, HostTypeListEnd>;
        type HostReturn = BigInt;
        type Arguments = (BigInt,);
        type Returned = BigInt;
        type Requirements = ProviderNoConstructions;

        fn into_host_arguments<'call>(
            arguments: Self::Arguments,
            _call: &mut HostCall<'call, TestHostProfile, Provider, ()>,
            _constructions: &ProviderConstructions<'call, Self::Requirements>,
        ) -> <Self::HostArguments as crate::HostTypeSequence>::Values<'call> {
            (arguments.0, ())
        }

        fn from_host_return<'call>(
            value: <Self::HostReturn as crate::HostType>::Value<'call>,
            _call: &mut HostCall<'call, TestHostProfile, Provider, ()>,
        ) -> Self::Returned {
            value
        }
    }

    #[test]
    fn shared_call_exposes_only_the_borrowed_provider_state() {
        let state = TestRunState {
            counter: 7,
            unrelated: true,
        };
        let call = Call::from_shared_state(&state);

        assert_eq!(call.state().counter, 7);
        assert!(call.state().unrelated);
    }

    #[test]
    fn active_call_projects_shared_and_mutable_provider_state() {
        let mut state = TestRunState::default();
        {
            let mut runtime =
                TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
            let host_call = HostCall::<TestHostProfile, Provider, bool>::new(&mut runtime);
            let mut call = Call::from_host_call(host_call);

            assert_eq!(call.state().counter, 0);
            call.state_mut().counter = 3;
            assert_eq!(call.state().counter, 3);

            let _recovered_call = call.into_host_call();
        }
        assert_eq!(state.counter, 3);
    }

    #[test]
    fn host_result_preserves_the_local_failure_envelope() {
        fn fail() -> HostResult<()> {
            Err(HostFailure::new("provider unavailable").into())
        }

        assert_eq!(
            fail()
                .expect_err("host failure should stop the call")
                .into_kind(),
            HostCallErrorKind::Failure(HostFailure::new("provider unavailable")),
        );
    }

    #[test]
    fn active_call_owns_generic_source_semantics_without_materializing_values() {
        type Parameter = HostTypeParameter<0>;
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        let left = Value::<Parameter, ProviderValueContext<Parameter>>::from_stored(
            crate::runtime::StoredRuntimeValue::test_int(7.into()),
        );
        let same = Value::<Parameter, ProviderValueContext<Parameter>>::from_stored(
            crate::runtime::StoredRuntimeValue::test_int(7.into()),
        );
        let different = Value::<Parameter, ProviderValueContext<Parameter>>::from_stored(
            crate::runtime::StoredRuntimeValue::test_int(8.into()),
        );
        let host_call = HostCall::<TestHostProfile, Provider, bool>::new(&mut runtime);
        let call = Call::from_host_call(host_call);

        assert!(call.equal(&left, &same));
        assert!(!call.equal(&left, &different));
        assert_eq!(call.source_hash(&left), call.source_hash(&same));
        assert_eq!(call.inspect(&left), "7");
    }

    #[test]
    fn owned_call_invokes_one_static_callback_codec_and_reenters_state() {
        type Context = ProviderOwnedCallbackContext<TestHostProfile, Provider, IntCallbackCodec>;
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        fn invoke<'call>(
            call: HostCall<'call, TestHostProfile, Provider, BigInt>,
            constructions: crate::HostConstructions<'call, HostTypeListEnd>,
            callback: HostCallable<'call, Arguments, BigInt>,
        ) -> Result<crate::HostCallContinuation<'call, BigInt>, crate::HostCallError> {
            let proof = ProviderConstructions::none();
            let callback =
                Callback::<fn(BigInt) -> BigInt, Context>::from_owned_host(&call, callback, proof);
            let alias = callback.clone();
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let mut call = Call::from_execution_context(context);
                    call.with_state(|state| state.counter += 1)
                        .await
                        .expect("the live entry services its state request");
                    let first = call.invoke(&callback, (BigInt::from(7),)).await?;
                    let second = call.invoke(&alias, (first,)).await?;
                    let counter = call
                        .with_call(|call| call.state().counter)
                        .await
                        .expect("the live entry services its value request");
                    assert_eq!(counter, 1);
                    Ok(crate::HostOwnedCompletion::new(move |call, _| {
                        Ok(call.return_value(second))
                    }))
                })
            }))
        }
        for (callback, expected, expected_echo) in [
            ("fn(value) { echo value value + 1 }", Ok(9), vec!["7", "8"]),
            (
                "fn(value) { echo value panic as \"first callback\" }",
                Err("first callback"),
                vec!["7"],
            ),
            (
                "fn(value) { echo value case value { 7 -> 8 _ -> panic as \"second callback\" } }",
                Err("second callback"),
                vec!["7", "8"],
            ),
        ] {
            let provider = crate::HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_function::<
                Provider,
                (crate::HostFunctionType<Arguments, BigInt>,),
                BigInt,
                HostTypeListEnd,
                _,
            >("invoke", invoke)
            .unwrap();
            let source = format!(
                r#"
@external(erlang, "native", "invoke")
fn invoke(callback: fn(Int) -> Int) -> Int
pub fn main() {{ invoke({callback}) }}
"#
            );
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [crate::PackageSource::new(
                    "application",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new("main", "main.gleam", source)],
                )],
                crate::HostProviderSet::from_providers([provider]).unwrap(),
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let mut state = TestRunState {
                counter: 0,
                unrelated: true,
            };
            let mut echo = Vec::new();
            let result = crate::execution_fixture::run(&mut execution, &mut state, &mut echo);
            match expected {
                Ok(value) => assert_eq!(result, Ok(crate::Value::Int(value.into()))),
                Err(message) => {
                    let error = result.unwrap_err();
                    assert_eq!(error.to_string(), format!("panic: {message}"));
                    let labels = miette::Diagnostic::labels(&error)
                        .unwrap()
                        .collect::<Vec<_>>();
                    assert_eq!(labels[0].label(), Some("panic in main.<anonymous:0>"));
                }
            }
            assert_eq!(state.counter, 1);
            assert!(state.unrelated);
            assert_eq!(
                echo.iter()
                    .map(|output| output.value().inspect().to_string())
                    .collect::<Vec<_>>(),
                expected_echo
            );
        }
    }
}
