use super::{
    Callback, ProviderCallbackCodec, ProviderCallbackContext, ProviderValueContext, Value,
};
use crate::{HostCall, HostCallError, HostProfile, HostProvider, HostType};
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
pub type HostResult<Value> = Result<Value, HostCallError>;

#[doc(hidden)]
pub struct ProviderCallPlaceholder;

#[doc(hidden)]
pub struct ProviderSharedCall<'state, State> {
    state: &'state State,
}

#[doc(hidden)]
pub struct ProviderActiveCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    call: HostCall<'call, Profile, Provider, Return>,
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

    /// Compares two generic values with Gleam source equality semantics.
    pub fn equal<Type, Host>(
        &self,
        left: &Value<Type, ProviderValueContext<'call, Host>>,
        right: &Value<Type, ProviderValueContext<'call, Host>>,
    ) -> bool
    where
        Host: HostType,
        Host::Value<'call>: Clone,
    {
        self.context.call.equal::<Host>(left.host(), right.host())
    }

    /// Hashes a generic call-scoped value consistently with source equality.
    ///
    /// The result is an execution-local lookup key, not a stable serialized
    /// value.
    pub fn source_hash<Type, Host>(
        &self,
        value: &Value<Type, ProviderValueContext<'call, Host>>,
    ) -> u64
    where
        Host: HostType,
        Host::Value<'call>: Clone,
    {
        self.context.call.source_hash::<Host>(value.host())
    }

    /// Returns the canonical source-facing inspection of a generic value.
    pub fn inspect<Type, Host>(
        &self,
        value: &Value<Type, ProviderValueContext<'call, Host>>,
    ) -> EcoString
    where
        Host: HostType,
        Host::Value<'call>: Clone,
    {
        self.context.call.inspect::<Host>(value.host())
    }

    /// Invokes one typed Gleam callback within this active provider call.
    pub fn invoke<Signature, Codec>(
        &mut self,
        callback: Callback<
            Signature,
            ProviderCallbackContext<'call, Profile, Provider, Return, Codec>,
        >,
        arguments: Codec::Arguments,
    ) -> HostResult<Codec::Returned>
    where
        Codec: ProviderCallbackCodec<'call, Profile, Provider, Return>,
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

#[cfg(test)]
mod tests {
    use super::{Call, HostResult};
    use crate::host::CallArguments;
    use crate::host::HostCallErrorKind;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCallable, HostFunctionToken, HostScopedValue, HostTypeList, HostTypeListEnd,
        HostTypeParameter, HostValue, HostValueFamily, HostValueToken,
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

    impl<'call> ProviderCallbackCodec<'call, TestHostProfile, Provider, BigInt> for IntCallbackCodec {
        type HostArguments = HostTypeList<BigInt, HostTypeListEnd>;
        type HostReturn = BigInt;
        type Arguments = (BigInt,);
        type Returned = BigInt;
        type Requirements = ProviderNoConstructions;

        fn into_host_arguments(
            arguments: Self::Arguments,
            _call: &mut HostCall<'call, TestHostProfile, Provider, BigInt>,
            _constructions: &ProviderConstructions<'call, Self::Requirements>,
        ) -> <Self::HostArguments as crate::HostTypeSequence>::Values<'call> {
            (arguments.0, ())
        }

        fn from_host_return(
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
        let host = HostValue::<Parameter>::new(HostValueToken {
            family: HostValueFamily::String,
            index: 2,
        });
        let left = Value::<Parameter, ProviderValueContext<'_, Parameter>>::from_host(host);
        let right = Value::<Parameter, ProviderValueContext<'_, Parameter>>::from_host(host);
        let host_call = HostCall::<TestHostProfile, Provider, bool>::new(&mut runtime);
        let call = Call::from_host_call(host_call);

        assert!(!call.equal(&left, &right));
        assert_eq!(call.source_hash(&left), 17);
        assert_eq!(call.inspect(&left), "inspected");
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
