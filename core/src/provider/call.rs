use super::{
    Callback, ProviderCallbackCodec, ProviderCallbackContext, ProviderExternalCodec,
    ProviderExternalView, ProviderFutureCallbackContext, ProviderOwnedStoredInput,
    ProviderStoredInput, ProviderStoredOutput, ProviderStoredOwner, ProviderValueContext,
    ProviderValueForms, Stored, Value,
};
use crate::host::{HostFutureContext, HostFutureError, HostTypeListEnd, HostTypeSequence};
use crate::provider::advanced::{
    ProviderDynamicInput, ProviderDynamicValue, Retained, StoredDynamic,
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

/// Request capability supplied to a macro-authored async provider function.
#[doc(hidden)]
pub struct ProviderFutureCall<'work, Profile, Provider>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    call: HostFutureContext<'work, Profile, Provider, HostTypeListEnd>,
}

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

    /// Invokes one typed Gleam callback during an immediate transferable call.
    pub fn invoke<Signature, Codec>(
        &mut self,
        callback: Callback<
            Signature,
            ProviderCallbackContext<'call, Profile, Provider, Return, Codec>,
        >,
        arguments: Codec::Arguments,
    ) -> Result<Codec::Returned, crate::HostCallError>
    where
        Codec: ProviderCallbackCodec<Profile, Provider, Return>,
    {
        callback.invoke(&mut self.context.call, arguments)
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

impl<'work, Profile, Provider> Call<Provider::State, ProviderFutureCall<'work, Profile, Provider>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
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
    ) -> Result<Output, HostFutureError>
    where
        Profile::RunState: Send,
        Operation: FnOnce(&mut Provider::State) -> Output + Send + 'static,
        Output: Send + 'static,
    {
        self.context.call.with_state(operation).await
    }

    /// Invokes a retained callback on its original execution, without implicitly driving its result.
    pub async fn invoke<Signature, Codec>(
        &mut self,
        callback: &Callback<Signature, ProviderFutureCallbackContext<Profile, Provider, Codec>>,
        arguments: Codec::Arguments,
    ) -> Result<Codec::Returned, HostFutureError>
    where
        Profile::RunState: Send,
        Profile::ExternalStores: Send,
        Codec: ProviderCallbackCodec<Profile, Provider, ()> + 'static,
        Codec::Arguments: Send + 'static,
        Codec::Returned: Send + 'static,
    {
        callback.invoke_future(&self.context.call, arguments).await
    }

    /// Explicitly observes a Future returned by Gleam, sharing its original work.
    pub async fn observe<Value, Host, Output>(
        &mut self,
        work: &super::Future<
            Value,
            super::ProviderFutureValueContext<Profile, Provider, Host, Output>,
        >,
    ) -> Result<Output, HostFutureError>
    where
        Host: HostType,
        Output: Send + 'static,
    {
        work.observe(&self.context.call).await
    }

    #[doc(hidden)]
    pub fn from_future_context<Constructions: HostTypeSequence>(
        call: HostFutureContext<'work, Profile, Provider, Constructions>,
    ) -> Self {
        Self {
            context: ProviderFutureCall {
                call: call.without_constructions(),
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
    use crate::host::{
        HostCallable, HostFunctionToken, HostScopedValue, HostTypeList, HostTypeListEnd,
        HostTypeParameter,
    };
    use crate::provider::{
        Callback, ProviderCallbackCodec, ProviderCallbackContext, ProviderConstructions,
        ProviderNoConstructions, ProviderValueContext, Value,
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

    impl ProviderCallbackCodec<TestHostProfile, Provider, BigInt> for IntCallbackCodec {
        type HostArguments = HostTypeList<BigInt, HostTypeListEnd>;
        type HostReturn = BigInt;
        type Arguments = (BigInt,);
        type Returned = BigInt;
        type Requirements = ProviderNoConstructions;

        fn into_host_arguments<'call>(
            arguments: Self::Arguments,
            _call: &mut HostCall<'call, TestHostProfile, Provider, BigInt>,
            _constructions: &ProviderConstructions<'call, Self::Requirements>,
        ) -> <Self::HostArguments as crate::HostTypeSequence>::Values<'call> {
            (arguments.0, ())
        }

        fn from_host_return<'call>(
            value: <Self::HostReturn as crate::HostType>::Value<'call>,
            _call: &mut HostCall<'call, TestHostProfile, Provider, BigInt>,
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
    fn active_call_invokes_one_static_callback_codec() {
        type Context<'call> =
            ProviderCallbackContext<'call, TestHostProfile, Provider, BigInt, IntCallbackCodec>;
        let mut state = TestRunState::default();
        let mut runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        let host_call = HostCall::<TestHostProfile, Provider, BigInt>::new(&mut runtime);
        let mut call = Call::from_host_call(host_call);
        let constructions = ProviderConstructions::none();
        let constructions = Clone::clone(&constructions);
        let callback = Callback::<fn(BigInt) -> BigInt, Context<'_>>::from_host(
            HostCallable::new(HostFunctionToken(3)),
            constructions,
        );
        let callback = Clone::clone(&callback);

        let returned = call
            .invoke(callback, (BigInt::from(7),))
            .expect("typed callback should invoke through the active call");
        assert_eq!(returned, BigInt::from(0));
        assert_eq!(runtime.completed(), Some(&HostScopedValue::Int(7.into())));
    }
}
