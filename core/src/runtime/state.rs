pub(in crate::runtime) mod list;

use crate::runtime::{LocalValues, RuntimeValueProfile};

pub(in crate::runtime) struct TransferRuntimeHost<'run, Profile: crate::HostProfile> {
    state: &'run mut Profile::RunState,
    stores: &'run Profile::ExternalStores,
    work: crate::runtime::work::execution::WorkContext<Profile>,
}

pub(in crate::runtime) trait RuntimeHostState {
    type State;

    fn state(&mut self) -> &mut Self::State;
}

impl RuntimeHostState for () {
    type State = ();

    fn state(&mut self) -> &mut Self::State {
        self
    }
}

impl<State> RuntimeHostState for &mut State {
    type State = State;

    fn state(&mut self) -> &mut Self::State {
        self
    }
}

impl<'run, Profile: crate::HostProfile> TransferRuntimeHost<'run, Profile> {
    pub(in crate::runtime) fn new(
        state: &'run mut Profile::RunState,
        stores: &'run Profile::ExternalStores,
        work: crate::runtime::work::execution::WorkContext<Profile>,
    ) -> Self {
        Self {
            state,
            stores,
            work,
        }
    }

    pub(in crate::runtime) fn stores(&self) -> &Profile::ExternalStores {
        self.stores
    }

    pub(in crate::runtime) fn work(
        &self,
    ) -> &crate::runtime::work::execution::WorkContext<Profile> {
        &self.work
    }
}

impl<Profile: crate::HostProfile> RuntimeHostState for TransferRuntimeHost<'_, Profile> {
    type State = Profile::RunState;

    fn state(&mut self) -> &mut Self::State {
        self.state
    }
}

pub(in crate::runtime) struct RuntimeState<'run, Host = (), Values = LocalValues>
where
    Values: RuntimeValueProfile,
{
    echo: &'run mut dyn crate::runtime::EchoSink,
    host: Host,
    lists: Values::ListStorage,
}

pub(in crate::runtime) type RuntimeStateFor<'run, Plan> = RuntimeState<
    'run,
    <Plan as crate::runtime::ExecutableRuntimePlan>::RuntimeHost<'run>,
    <Plan as crate::plan::execution::runtime::RuntimeExecutionPlan>::Values,
>;

impl<'run> RuntimeState<'run, (), LocalValues> {
    pub(super) fn new(echo: &'run mut dyn crate::runtime::EchoSink) -> Self {
        Self {
            echo,
            host: (),
            lists: Default::default(),
        }
    }
}

impl<'run, Host, Values> RuntimeState<'run, Host, Values>
where
    Values: RuntimeValueProfile,
{
    pub(super) fn with_host(echo: &'run mut dyn crate::runtime::EchoSink, host: Host) -> Self {
        Self {
            echo,
            host,
            lists: Default::default(),
        }
    }

    pub(super) fn with_host_and_lists(
        echo: &'run mut dyn crate::runtime::EchoSink,
        host: Host,
        lists: Values::ListStorage,
    ) -> Self {
        Self { echo, host, lists }
    }

    pub(super) fn host(&self) -> &Host {
        &self.host
    }

    pub(super) fn emit_echo(&mut self, output: crate::runtime::EchoOutput) {
        self.echo.emit(output);
    }

    pub(super) fn lists(&self) -> &Values::ListStorage {
        &self.lists
    }

    pub(super) fn lists_mut(&mut self) -> &mut Values::ListStorage {
        &mut self.lists
    }
}

impl<Host, Values> RuntimeState<'_, Host, Values>
where
    Host: RuntimeHostState,
    Values: RuntimeValueProfile,
{
    pub(super) fn host_state(&mut self) -> &mut Host::State {
        self.host.state()
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeState;

    #[test]
    fn runtime_state_exposes_owned_and_borrowed_host_state() {
        let mut echo = Vec::new();
        let mut plain = RuntimeState::new(&mut echo);

        assert_eq!(plain.host_state(), &mut ());

        let mut host = (num_bigint::BigInt::from(41), true);
        let mut echo = Vec::new();
        let mut hosted: RuntimeState<'_, _, crate::runtime::LocalValues> =
            RuntimeState::with_host(&mut echo, &mut host);
        hosted.host_state().0 += 1;

        assert!(hosted.host_state().1);

        drop(hosted);
        assert_eq!(host, (num_bigint::BigInt::from(42), true));
    }
}
