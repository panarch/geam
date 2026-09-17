use crate::runtime::state::RuntimeState;
use crate::runtime::{EchoOutput, RuntimeListStorage};

pub(in crate::runtime) struct Evaluation {
    lists: RuntimeListStorage,
    captures: crate::runtime::CaptureStorage,
    echo: Vec<EchoOutput>,
}

impl Evaluation {
    pub(in crate::runtime) fn new(captures: crate::runtime::CaptureStorage) -> Self {
        Self {
            lists: RuntimeListStorage::default(),
            captures,
            echo: Vec::new(),
        }
    }

    pub(in crate::runtime) fn access<Output>(
        &mut self,
        evaluate: impl FnOnce(&mut RuntimeState<'_>) -> Output,
    ) -> Output {
        let mut state = RuntimeState::with_host_storage(
            &mut self.echo,
            (),
            self.lists.clone(),
            self.captures.clone(),
        );
        evaluate(&mut state)
    }

    pub(in crate::runtime) fn take_echo(&mut self) -> Vec<EchoOutput> {
        std::mem::take(&mut self.echo)
    }
}
