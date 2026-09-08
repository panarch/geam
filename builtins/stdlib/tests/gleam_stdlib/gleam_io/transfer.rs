use super::super::transfer_fixture::{ENTRY, TransferFixture, observed_project};
use super::RecordedEvent;
use geam_core::host::{AsyncHostComponentProfile, HostFutureStore};
use geam_core::{EchoOutput, EchoSink, HostProfile, TransferHostProviderSet};
use geam_runtime_api::FutureComponent;
use geam_stdlib::{
    Component, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibTransferStores, IoOutput,
    IoSink, IoStream,
};
use std::cell::Cell;
use std::sync::{Arc, Mutex};

#[test]
fn preserves_io_echo_and_failure_with_a_send_only_caller_sink() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed_with_io(
            [9; 32],
            Sink {
                events: Arc::clone(&events),
                count: Cell::new(0),
            },
        ),
        work: (),
    };
    let mut echo = Echo(Arc::clone(&events));
    let program = observed_project(
        &super::project_root(),
        "gleam_io_order_and_panic",
        TransferHostProviderSet::new(
            geam_stdlib::transfer_host_providers::<Profile>().expect("IO transfer registration"),
        )
        .expect("IO transfer set"),
    );
    let mut execution = TransferFixture::new(program, ENTRY);
    let error = execution
        .run(&mut state, &mut echo)
        .expect_err("source panic after IO");
    let geam_core::embedding::AsyncCallError::Execution(geam_core::AsyncExecutionError::Panic(
        panic,
    )) = error
    else {
        panic!("source panic must retain its original domain");
    };
    assert_eq!(panic.kind(), geam_core::PanicKind::Panic);
    assert_eq!(panic.site().module(), "gleam_io_order_and_panic");
    assert_eq!(panic.site().function(), "main");
    assert_eq!(
        panic.message(),
        &geam_core::PanicMessage::Explicit("stop".into())
    );
    assert_eq!(
        events.lock().expect("event lock").as_slice(),
        [
            RecordedEvent::Io(IoStream::Stdout, "before".into()),
            RecordedEvent::Io(IoStream::Stdout, "stdout line\n".into()),
            RecordedEvent::Echo {
                message: Some("between".into()),
                value: "Nil".into()
            },
            RecordedEvent::Io(IoStream::Stderr, "after".into()),
            RecordedEvent::Io(IoStream::Stderr, "stderr line\n".into()),
        ]
    );
}

struct Profile;
struct State {
    stdlib: GleamStdlibRunState<Sink>,
    work: (),
}
#[derive(Default)]
struct Stores {
    stdlib: GleamStdlibTransferStores,
    work: HostFutureStore,
}
struct Sink {
    events: Arc<Mutex<Vec<RecordedEvent>>>,
    count: Cell<usize>,
}
struct Echo(Arc<Mutex<Vec<RecordedEvent>>>);

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
}
impl GleamStdlibHostProfile for Profile {
    type Io = Sink;
}
impl AsyncHostComponentProfile<Component<Sink>> for Profile {
    fn component_async_stores(stores: &Stores) -> &GleamStdlibTransferStores {
        &stores.stdlib
    }
    fn component_state(state: &mut State) -> &mut GleamStdlibRunState<Sink> {
        &mut state.stdlib
    }
}
impl geam_core::host::HostWorkProfile for Profile {
    type Work = FutureComponent;
}
impl AsyncHostComponentProfile<FutureComponent> for Profile {
    fn component_async_stores(stores: &Stores) -> &HostFutureStore {
        &stores.work
    }
    fn component_state(state: &mut State) -> &mut () {
        &mut state.work
    }
}
impl IoSink for Sink {
    fn emit(&mut self, output: IoOutput) {
        self.count.set(self.count.get() + 1);
        self.events
            .lock()
            .expect("event lock")
            .push(RecordedEvent::Io(output.stream(), output.text().clone()));
    }
}
impl EchoSink for Echo {
    fn emit(&mut self, output: EchoOutput) {
        self.0
            .lock()
            .expect("event lock")
            .push(RecordedEvent::Echo {
                message: output.message().cloned(),
                value: output.value().inspect().to_string().into(),
            });
    }
}
