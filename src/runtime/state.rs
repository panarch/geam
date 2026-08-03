pub(in crate::runtime) mod list;

use self::list::RuntimeListStorage;

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

pub(in crate::runtime) struct RuntimeState<'run, Host = ()> {
    echo: &'run mut dyn crate::runtime::EchoSink,
    host: Host,
    lists: RuntimeListStorage,
}

pub(in crate::runtime) type RuntimeStateFor<'run, Plan> =
    RuntimeState<'run, <Plan as crate::runtime::ExecutableRuntimePlan>::RuntimeHost<'run>>;

impl<'run> RuntimeState<'run, ()> {
    pub(super) fn new(echo: &'run mut dyn crate::runtime::EchoSink) -> Self {
        Self {
            echo,
            host: (),
            lists: RuntimeListStorage::default(),
        }
    }
}

impl<'run, Host> RuntimeState<'run, Host> {
    pub(super) fn with_host(echo: &'run mut dyn crate::runtime::EchoSink, host: Host) -> Self {
        Self {
            echo,
            host,
            lists: RuntimeListStorage::default(),
        }
    }

    pub(super) fn emit_echo(&mut self, output: crate::runtime::EchoOutput) {
        self.echo.emit(output);
    }

    pub(super) fn lists(&self) -> &RuntimeListStorage {
        &self.lists
    }

    pub(super) fn lists_mut(&mut self) -> &mut RuntimeListStorage {
        &mut self.lists
    }
}

impl<Host: RuntimeHostState> RuntimeState<'_, Host> {
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
        let mut hosted = RuntimeState::with_host(&mut echo, &mut host);
        hosted.host_state().0 += 1;

        assert!(hosted.host_state().1);

        drop(hosted);
        assert_eq!(host, (num_bigint::BigInt::from(42), true));
    }
}
