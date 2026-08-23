use crate::{HostCall, HostCallError, HostProfile, HostProvider, HostType};
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
    use crate::{HostCall, HostFailure, HostProvider};

    struct Provider;

    impl HostProvider<TestHostProfile> for Provider {
        type State = TestRunState;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            state
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
}
