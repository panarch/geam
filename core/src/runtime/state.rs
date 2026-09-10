pub(in crate::runtime) mod list;

pub(in crate::runtime) struct RuntimeHost<'run, Profile: crate::HostProfile> {
    state: &'run mut Profile::RunState,
    stores: &'run Profile::ExternalStores,
    work: &'run crate::runtime::work::execution::ExecutionWork<Profile>,
    units: &'run mut crate::runtime::execution::Units<Profile>,
    execution: crate::runtime::execution::ExecutionContext<Profile>,
    clock: crate::execution::ExecutionClock<'run>,
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

impl<'run, Profile: crate::HostProfile> RuntimeHost<'run, Profile> {
    pub(in crate::runtime) fn new(
        state: &'run mut Profile::RunState,
        stores: &'run Profile::ExternalStores,
        work: &'run crate::runtime::work::execution::ExecutionWork<Profile>,
        units: &'run mut crate::runtime::execution::Units<Profile>,
        execution: crate::runtime::execution::ExecutionContext<Profile>,
        clock: crate::execution::ExecutionClock<'run>,
    ) -> Self {
        Self {
            state,
            stores,
            work,
            units,
            execution,
            clock,
        }
    }

    pub(in crate::runtime) fn stores(&self) -> &Profile::ExternalStores {
        self.stores
    }

    pub(in crate::runtime) fn work(&self) -> crate::runtime::work::execution::WorkContext<Profile> {
        self.work.context()
    }

    pub(in crate::runtime) fn execution(
        &self,
    ) -> crate::runtime::execution::ExecutionContext<Profile> {
        self.execution.clone()
    }

    pub(in crate::runtime) fn execution_state(&mut self) -> &mut Profile::ExecutionState {
        self.units.state()
    }

    pub(in crate::runtime) fn clock(&self) -> crate::execution::ExecutionClock<'_> {
        self.clock
    }

    pub(in crate::runtime) fn spawn(
        &mut self,
        callable: crate::runtime::RetainedCallable,
        origin: crate::runtime::HostCallOrigin,
    ) -> crate::execution::ExecutionUnit {
        self.units.spawn(callable, origin)
    }
}

impl<Profile: crate::HostProfile> RuntimeHostState for RuntimeHost<'_, Profile> {
    type State = Profile::RunState;

    fn state(&mut self) -> &mut Self::State {
        self.state
    }
}

pub(in crate::runtime) struct RuntimeState<'run, Host = ()> {
    echo: &'run mut dyn crate::runtime::EchoSink,
    host: Host,
    lists: crate::runtime::RuntimeListStorage,
}

pub(in crate::runtime) type RuntimeStateFor<'run, Plan> =
    RuntimeState<'run, <Plan as crate::runtime::ExecutableRuntimePlan>::RuntimeHost<'run>>;

impl<'run> RuntimeState<'run, ()> {
    pub(super) fn new(echo: &'run mut dyn crate::runtime::EchoSink) -> Self {
        Self {
            echo,
            host: (),
            lists: Default::default(),
        }
    }
}

impl<'run, Host> RuntimeState<'run, Host> {
    pub(super) fn with_host_and_lists(
        echo: &'run mut dyn crate::runtime::EchoSink,
        host: Host,
        lists: crate::runtime::RuntimeListStorage,
    ) -> Self {
        Self { echo, host, lists }
    }

    pub(super) fn host(&self) -> &Host {
        &self.host
    }

    pub(super) fn host_mut(&mut self) -> &mut Host {
        &mut self.host
    }

    pub(super) fn host_and_lists(&mut self) -> (&mut Host, &crate::runtime::RuntimeListStorage) {
        (&mut self.host, &self.lists)
    }

    pub(super) fn emit_echo(&mut self, output: crate::runtime::EchoOutput) {
        self.echo.emit(output);
    }

    pub(super) fn lists(&self) -> &crate::runtime::RuntimeListStorage {
        &self.lists
    }

    pub(super) fn lists_mut(&mut self) -> &mut crate::runtime::RuntimeListStorage {
        &mut self.lists
    }
}

impl<Host> RuntimeState<'_, Host>
where
    Host: RuntimeHostState,
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
        let mut hosted: RuntimeState<'_, _> =
            RuntimeState::with_host_and_lists(&mut echo, &mut host, Default::default());
        hosted.host_state().0 += 1;

        assert!(hosted.host_state().1);

        drop(hosted);
        assert_eq!(host, (num_bigint::BigInt::from(42), true));
    }
}
