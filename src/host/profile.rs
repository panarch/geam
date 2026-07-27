use std::marker::PhantomData;

pub trait HostProfile: Send + Sync + 'static {
    type RunState;
}

pub trait HostProvider<Profile: HostProfile>: Send + Sync + 'static {
    type State;

    fn project(state: &mut Profile::RunState) -> &mut Self::State;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatelessHostProfile;

pub struct HostCall<'call, Profile, Provider>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    state: &'call mut Profile::RunState,
    provider: PhantomData<Provider>,
}

impl HostProfile for StatelessHostProfile {
    type RunState = ();
}

impl<'call, Profile, Provider> HostCall<'call, Profile, Provider>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
{
    pub(crate) fn new(state: &'call mut Profile::RunState) -> Self {
        Self {
            state,
            provider: PhantomData,
        }
    }

    pub fn state(&mut self) -> &mut Provider::State {
        Provider::project(self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostCall, HostProfile, HostProvider};

    struct Profile;

    struct RunState {
        counter: usize,
        unrelated: bool,
    }

    struct Counter;

    impl HostProfile for Profile {
        type RunState = RunState;
    }

    impl HostProvider<Profile> for Counter {
        type State = usize;

        fn project(state: &mut RunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    #[test]
    fn host_call_exposes_only_the_selected_provider_state() {
        let mut state = RunState {
            counter: 1,
            unrelated: true,
        };

        *HostCall::<Profile, Counter>::new(&mut state).state() += 1;

        assert_eq!(state.counter, 2);
        assert!(state.unrelated);
    }
}
