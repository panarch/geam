use ecow::EcoString;
use geam_core::{
    EchoOutput, EchoSink, ExecutionError, HostComponentProfile, HostModule, HostProfile,
    HostProviderSet, HostedExecution, PanicKind, PanicMessage, Value, compile_typed_host_project,
    plan_host_program,
};
use geam_stdlib::{
    Component, GleamStdlibHostProfile, GleamStdlibProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput, IoSink, IoStream, host_providers,
};
use std::sync::{Arc, Mutex};

use super::{ExpectedSurface, assert_surface, project_root};

#[path = "gleam_io/transfer.rs"]
mod transfer;

const DEPENDENCIES: &[&str] = &["gleam/io"];

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &["print", "print_error", "println", "println_error"],
    types: &[],
    type_aliases: &[],
    constructors: &[],
    functions: r#"
print: fn(String) -> Nil
print_error: fn(String) -> Nil
println: fn(String) -> Nil
println_error: fn(String) -> Nil
"#,
};

const EXPECTED_OUTPUTS: &[(IoStream, &str)] = &[
    (IoStream::Stdout, "stdout"),
    (IoStream::Stderr, "stderr"),
    (IoStream::Stdout, "stdout line\n"),
    (IoStream::Stderr, "stderr line\n"),
    (IoStream::Stdout, ""),
    (IoStream::Stdout, "\n"),
    (IoStream::Stderr, "embedded\n"),
    (IoStream::Stderr, "embedded\n\n"),
];

#[test]
fn tracks_official_gleam_io_public_surface() {
    assert_surface("gleam_io", "gleam/io", DEPENDENCIES, &SURFACE);
}

#[test]
fn runs_official_gleam_io_with_caller_owned_output() {
    let execution = execution::<GleamStdlibProfile>("gleam_io");
    let mut repeated_state = GleamStdlibRunState::from_seed([7; 32]);

    for _ in 0..2 {
        assert_eq!(
            execution
                .run_main(&mut repeated_state, &mut Vec::new())
                .expect("official IO source should run"),
            Value::Nil,
        );
    }

    assert_outputs(
        repeated_state.io_outputs(),
        EXPECTED_OUTPUTS
            .iter()
            .copied()
            .chain(EXPECTED_OUTPUTS.iter().copied()),
    );
    let taken = repeated_state.take_io_outputs();
    assert_outputs(
        &taken,
        EXPECTED_OUTPUTS
            .iter()
            .copied()
            .chain(EXPECTED_OUTPUTS.iter().copied()),
    );
    assert!(repeated_state.io_outputs().is_empty());

    let mut independent_state = GleamStdlibRunState::from_seed([8; 32]);
    assert_eq!(
        execution
            .run_main(&mut independent_state, &mut Vec::new())
            .expect("official IO source should run with independent state"),
        Value::Nil,
    );
    assert_outputs(
        independent_state.io_outputs(),
        EXPECTED_OUTPUTS.iter().copied(),
    );

    let mut transferred = super::transfer::fixture("gleam_io");
    let mut transfer_state = super::transfer_support::RunState {
        stdlib: GleamStdlibRunState::from_seed([7; 32]),
        work: (),
    };
    let mut transfer_echo = super::transfer_fixture::ObservedEcho::default();
    for _ in 0..2 {
        transferred
            .run(&mut transfer_state, &mut transfer_echo)
            .expect("transfer IO repeated execution");
        std::mem::take(&mut transfer_echo).assert_result(&Value::Nil, &[]);
    }
    assert_outputs(
        transfer_state.stdlib.io_outputs(),
        EXPECTED_OUTPUTS
            .iter()
            .copied()
            .chain(EXPECTED_OUTPUTS.iter().copied()),
    );
    let taken = transfer_state.stdlib.take_io_outputs();
    assert_outputs(
        &taken,
        EXPECTED_OUTPUTS
            .iter()
            .copied()
            .chain(EXPECTED_OUTPUTS.iter().copied()),
    );
    assert!(transfer_state.stdlib.io_outputs().is_empty());
    let mut independent = super::transfer_support::RunState {
        stdlib: GleamStdlibRunState::from_seed([8; 32]),
        work: (),
    };
    transferred
        .run(&mut independent, &mut transfer_echo)
        .expect("transfer IO independent state");
    transfer_echo.assert_result(&Value::Nil, &[]);
    assert_outputs(
        independent.stdlib.io_outputs(),
        EXPECTED_OUTPUTS.iter().copied(),
    );
}

#[test]
fn preserves_io_and_echo_order_before_a_later_panic() {
    let execution = execution::<RecordingProfile>("gleam_io_order_and_panic");
    let events = Arc::new(Mutex::new(Vec::new()));
    let mut state = RecordingRunState {
        stdlib: GleamStdlibRunState::from_seed_with_io(
            [9; 32],
            RecordingIoSink {
                events: Arc::clone(&events),
            },
        ),
    };
    let mut echo = RecordingEchoSink {
        events: Arc::clone(&events),
    };

    let error = execution
        .run_main(&mut state, &mut echo)
        .expect_err("fixture should panic after emitting its events");
    let ExecutionError::Panic(panic) = error else {
        panic!("fixture should preserve its source panic");
    };

    assert_eq!(panic.kind(), PanicKind::Panic);
    assert_eq!(panic.message(), &PanicMessage::Explicit("stop".into()));
    assert_eq!(
        events.lock().expect("event lock").as_slice(),
        [
            RecordedEvent::Io(IoStream::Stdout, "before".into()),
            RecordedEvent::Io(IoStream::Stdout, "stdout line\n".into()),
            RecordedEvent::Echo {
                message: Some("between".into()),
                value: "Nil".into(),
            },
            RecordedEvent::Io(IoStream::Stderr, "after".into()),
            RecordedEvent::Io(IoStream::Stderr, "stderr line\n".into()),
        ],
    );
}

fn execution<Profile>(root_module: &str) -> HostedExecution<Profile>
where
    Profile: geam_stdlib::GleamStdlibProviderProfile,
{
    let providers = host_providers::<Profile>().expect("official stdlib providers should register");
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
        .expect("official stdlib provider modules should be unique");
    let typed = compile_typed_host_project(project_root(), root_module, hosts)
        .expect("official IO fixture should compile");
    let plan = plan_host_program(typed).expect("official IO fixture should plan");

    assert_eq!(
        plan.modules()
            .iter()
            .map(|module| (module.package().as_str(), module.module().as_str()))
            .collect::<Vec<_>>(),
        [
            ("gleam_stdlib", "gleam/io"),
            ("geam_stdlib_test", root_module),
        ],
    );

    HostedExecution::try_from_module_plan(plan).expect("official IO fixture should seal")
}

fn assert_outputs<'expected>(
    outputs: &[IoOutput],
    expected: impl IntoIterator<Item = (IoStream, &'expected str)>,
) {
    assert_eq!(
        outputs
            .iter()
            .map(|output| (output.stream(), output.text().as_str()))
            .collect::<Vec<_>>(),
        expected.into_iter().collect::<Vec<_>>(),
    );
}

struct RecordingProfile;

struct RecordingRunState {
    stdlib: GleamStdlibRunState<RecordingIoSink>,
}

#[derive(Default)]
struct RecordingStores {
    stdlib: GleamStdlibStores,
}

struct RecordingIoSink {
    events: Arc<Mutex<Vec<RecordedEvent>>>,
}

struct RecordingEchoSink {
    events: Arc<Mutex<Vec<RecordedEvent>>>,
}

#[derive(Debug, PartialEq, Eq)]
enum RecordedEvent {
    Io(IoStream, EcoString),
    Echo {
        message: Option<EcoString>,
        value: EcoString,
    },
}

impl HostProfile for RecordingProfile {
    type RunState = RecordingRunState;
    type ExternalStores = RecordingStores;
}

impl HostComponentProfile<Component<RecordingIoSink>> for RecordingProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState<RecordingIoSink> {
        &mut state.stdlib
    }
}

impl GleamStdlibHostProfile for RecordingProfile {
    type Io = RecordingIoSink;
}

impl IoSink for RecordingIoSink {
    fn emit(&mut self, output: IoOutput) {
        self.events
            .lock()
            .expect("event lock")
            .push(RecordedEvent::Io(output.stream(), output.text().clone()));
    }
}

impl EchoSink for RecordingEchoSink {
    fn emit(&mut self, output: EchoOutput) {
        self.events
            .lock()
            .expect("event lock")
            .push(RecordedEvent::Echo {
                message: output.message().cloned(),
                value: output.value().inspect().to_string().into(),
            });
    }
}
