use crate::runtime::state::RuntimeState;
use crate::runtime::{EchoOutput, RuntimeListStorage};

#[derive(Default)]
pub(in crate::runtime) struct Evaluation {
    lists: RuntimeListStorage,
    echo: Vec<EchoOutput>,
}

impl Evaluation {
    pub(in crate::runtime) fn access<Output>(
        &mut self,
        evaluate: impl FnOnce(&mut RuntimeState<'_>) -> Output,
    ) -> Output {
        let mut state = RuntimeState::with_host_and_lists(&mut self.echo, (), self.lists.clone());
        evaluate(&mut state)
    }

    pub(in crate::runtime) fn take_echo(&mut self) -> Vec<EchoOutput> {
        std::mem::take(&mut self.echo)
    }
}
