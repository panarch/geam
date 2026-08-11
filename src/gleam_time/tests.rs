use super::{
    Component, GleamTimeHostProfile, GleamTimeProfile, GleamTimeProfileStores, GleamTimeRunState,
    TimeProvider, TimeSource, host_providers,
};
use crate::gleam_stdlib::{
    Component as GleamStdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState,
    GleamStdlibStores, IoOutput, IoSink,
};
use crate::{
    ExecutionError, HostComponentProfile, HostFailure, HostModule, HostProfile, HostProvider,
    HostProviderComponent, HostProviderComponentRegistration, HostProviderSet, HostedExecution,
    ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
};
use ecow::EcoString;
use std::collections::VecDeque;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct CustomProfile;

#[derive(Default)]
struct CustomStores {
    stdlib: GleamStdlibStores,
    time: (),
}

struct CustomRunState {
    stdlib: GleamStdlibRunState<RecordingSink>,
    source: ScriptedSource,
}

#[derive(Default)]
struct RecordingSink {
    outputs: Vec<IoOutput>,
}

#[derive(Default)]
struct ScriptedSource {
    times: VecDeque<Result<SystemTime, HostFailure>>,
    offsets: VecDeque<Result<i32, HostFailure>>,
}

impl IoSink for RecordingSink {
    fn emit(&mut self, output: IoOutput) {
        self.outputs.push(output);
    }
}

impl TimeSource for ScriptedSource {
    fn system_time(&mut self) -> Result<SystemTime, HostFailure> {
        self.times
            .pop_front()
            .unwrap_or_else(|| Err(HostFailure::new("scripted system time exhausted")))
    }

    fn local_offset_seconds(&mut self) -> Result<i32, HostFailure> {
        self.offsets
            .pop_front()
            .unwrap_or_else(|| Err(HostFailure::new("scripted local offset exhausted")))
    }
}

impl HostProfile for CustomProfile {
    type RunState = CustomRunState;
    type ExternalStores = CustomStores;
}

impl HostComponentProfile<GleamStdlibComponent<RecordingSink>> for CustomProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &GleamStdlibStores {
        &stores.stdlib
    }

    fn component_state(state: &mut Self::RunState) -> &mut GleamStdlibRunState<RecordingSink> {
        &mut state.stdlib
    }
}

impl GleamStdlibHostProfile for CustomProfile {
    type Io = RecordingSink;
}

impl HostComponentProfile<Component<ScriptedSource>> for CustomProfile {
    fn component_stores(stores: &Self::ExternalStores) -> &() {
        &stores.time
    }

    fn component_state(state: &mut Self::RunState) -> &mut ScriptedSource {
        &mut state.source
    }
}

impl GleamTimeHostProfile for CustomProfile {
    type Source = ScriptedSource;
}

const CALENDAR_SOURCE: &str = r#"
@external(erlang, "gleam_time_ffi", "local_time_offset_seconds")
@external(javascript, "../../gleam_time_ffi.mjs", "local_time_offset_seconds")
fn local_time_offset_seconds() -> Int

pub fn current_offset() -> Int {
  local_time_offset_seconds()
}
"#;

const TIMESTAMP_SOURCE: &str = r#"
@external(erlang, "gleam_time_ffi", "system_time")
@external(javascript, "../../gleam_time_ffi.mjs", "system_time")
fn get_system_time() -> #(Int, Int)

pub fn current_parts() -> #(Int, Int) {
  get_system_time()
}
"#;

const MAIN_SOURCE: &str = r#"
import gleam/time/calendar
import gleam/time/timestamp

pub fn main() {
  #(
    timestamp.current_parts(),
    calendar.current_offset(),
    timestamp.current_parts(),
    calendar.current_offset(),
  )
}
"#;

#[test]
fn default_and_custom_profiles_project_owned_state_stores_source_and_io() {
    let default_stores = GleamTimeProfileStores::default();
    let custom_stores = CustomStores::default();
    let mut default_state = GleamTimeRunState::new(
        GleamStdlibRunState::from_seed([1; 32]),
        ScriptedSource::default(),
    );
    let mut custom_state = CustomRunState {
        stdlib: GleamStdlibRunState::from_seed_with_io([2; 32], RecordingSink::default()),
        source: ScriptedSource::default(),
    };

    assert!(std::ptr::eq(
        <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
            GleamStdlibComponent,
        >>::component_stores(&default_stores),
        &default_stores.stdlib,
    ));
    assert!(std::ptr::eq(
        <CustomProfile as HostComponentProfile<GleamStdlibComponent<RecordingSink>>>::component_stores(
            &custom_stores,
        ),
        &custom_stores.stdlib,
    ));
    let default_stdlib = default_state.stdlib() as *const GleamStdlibRunState;
    assert!(std::ptr::eq(
        <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
            GleamStdlibComponent,
        >>::component_state(&mut default_state),
        default_stdlib,
    ));
    let custom_stdlib = &custom_state.stdlib as *const GleamStdlibRunState<RecordingSink>;
    assert!(std::ptr::eq(
        <CustomProfile as HostComponentProfile<
            GleamStdlibComponent<RecordingSink>,
        >>::component_state(&mut custom_state),
        custom_stdlib,
    ));

    let default_source = default_state.source() as *const ScriptedSource;
    assert!(std::ptr::eq(
        <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
            Component<ScriptedSource>,
        >>::component_state(&mut default_state),
        default_source,
    ));
    let custom_source = &custom_state.source as *const ScriptedSource;
    assert!(std::ptr::eq(
        <CustomProfile as HostComponentProfile<Component<ScriptedSource>>>::component_state(
            &mut custom_state,
        ),
        custom_source,
    ));
    assert!(std::ptr::eq(
        <GleamTimeProfile<ScriptedSource> as HostComponentProfile<
            Component<ScriptedSource>,
        >>::component_stores(&default_stores),
        &default_stores.time,
    ));
    assert!(std::ptr::eq(
        <CustomProfile as HostComponentProfile<Component<ScriptedSource>>>::component_stores(
            &custom_stores,
        ),
        &custom_stores.time,
    ));
    let provider_source = default_state.source() as *const ScriptedSource;
    assert!(std::ptr::eq(
        <TimeProvider<GleamTimeProfile<ScriptedSource>> as HostProvider<
            GleamTimeProfile<ScriptedSource>,
        >>::project(&mut default_state),
        provider_source,
    ));

    assert!(default_state.stdlib_mut().take_io_outputs().is_empty());
    assert!(default_state.stdlib().io_outputs().is_empty());
    default_state.source_mut().offsets.push_back(Ok(3600));

    assert!(default_state.stdlib().io_outputs().is_empty());
    assert_eq!(default_state.source().offsets.len(), 1);
}

#[test]
fn registers_calendar_then_timestamp_without_external_types() {
    assert_eq!(<Component as HostProviderComponent>::ID, "gleam_time");
    let providers = <Component as HostProviderComponentRegistration<GleamTimeProfile>>::providers()
        .expect("Time component should register");
    let facade =
        host_providers::<GleamTimeProfile>().expect("official Time providers should register");
    assert_eq!(
        facade
            .iter()
            .map(|provider| provider.module().as_str())
            .collect::<Vec<_>>(),
        providers
            .iter()
            .map(|provider| provider.module().as_str())
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        providers
            .iter()
            .map(|provider| (provider.package().as_str(), provider.module().as_str()))
            .collect::<Vec<_>>(),
        [
            ("gleam_time", "gleam/time/calendar"),
            ("gleam_time", "gleam/time/timestamp"),
        ],
    );
    assert!(
        providers
            .iter()
            .all(|provider| provider.external_types().count() == 0),
    );
}

#[test]
fn executes_non_monotonic_time_and_changing_offsets_in_source_order() {
    let execution = execution::<ScriptedSource>(MAIN_SOURCE, "main");
    let mut first_state = scripted_state();
    let mut independent_state = scripted_state();

    let first = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("scripted Time source should run");
    let repeated = execution
        .run_main(&mut first_state, &mut Vec::new())
        .expect("scripted Time source should run repeatedly");
    let independent = execution
        .run_main(&mut independent_state, &mut Vec::new())
        .expect("independent scripted Time source should run");

    assert_eq!(
        first.inspect().to_string(),
        "#(#(5, 0), 3600, #(-1, 999999999), -18000)",
    );
    assert_eq!(
        repeated.inspect().to_string(),
        "#(#(8, 7), 7200, #(-3, 999999997), 0)",
    );
    assert_eq!(independent, first);
    assert!(first_state.source().times.is_empty());
    assert!(first_state.source().offsets.is_empty());
}

#[test]
fn preserves_calendar_and_timestamp_source_failures() {
    for (module, source, failure, expected_function, expected_failure, expected_line) in [
        (
            "gleam/time/calendar",
            format!("{CALENDAR_SOURCE}\npub fn main() {{\n  current_offset()\n}}\n",),
            ScriptedSource {
                times: VecDeque::new(),
                offsets: [Err(HostFailure::new("offset unavailable"))].into(),
            },
            "local_time_offset_seconds",
            "offset unavailable",
            7,
        ),
        (
            "gleam/time/timestamp",
            format!("{TIMESTAMP_SOURCE}\npub fn main() {{\n  current_parts()\n}}\n",),
            ScriptedSource {
                times: [Err(HostFailure::new("clock unavailable"))].into(),
                offsets: VecDeque::new(),
            },
            "get_system_time",
            "clock unavailable",
            7,
        ),
    ] {
        let execution = execution::<ScriptedSource>(&source, module);
        let mut state = GleamTimeRunState::new(GleamStdlibRunState::from_seed([4; 32]), failure);
        let error = execution
            .run_main(&mut state, &mut Vec::new())
            .expect_err("scripted Time failure should remain an execution error");
        let ExecutionError::Host(error) = error else {
            panic!("scripted Time failure should remain a host error");
        };

        assert_eq!(error.package(), "gleam_time");
        assert_eq!(error.module(), module);
        assert_eq!(error.function(), expected_function);
        assert_eq!(error.failure().message(), expected_failure);
        assert_eq!(
            error
                .location()
                .path()
                .expect("synthetic Time failure should retain its source path")
                .as_str(),
            format!("src/{module}.gleam"),
        );
        assert_eq!(error.location().line(), Some(expected_line));
    }
}

fn scripted_state() -> GleamTimeRunState<ScriptedSource> {
    GleamTimeRunState::new(
        GleamStdlibRunState::from_seed([3; 32]),
        ScriptedSource {
            times: [
                Ok(UNIX_EPOCH + Duration::from_secs(5)),
                Ok(UNIX_EPOCH - Duration::from_nanos(1)),
                Ok(UNIX_EPOCH + Duration::new(8, 7)),
                Ok(UNIX_EPOCH - Duration::new(2, 3)),
            ]
            .into(),
            offsets: [Ok(3600), Ok(-18_000), Ok(7200), Ok(0)].into(),
        },
    )
}

fn execution<Source>(source: &str, root_module: &str) -> HostedExecution<GleamTimeProfile<Source>>
where
    Source: TimeSource,
{
    let modules = if root_module == "main" {
        vec![
            ModuleSource::new(
                "gleam/time/calendar",
                "src/gleam/time/calendar.gleam",
                CALENDAR_SOURCE,
            ),
            ModuleSource::new(
                "gleam/time/timestamp",
                "src/gleam/time/timestamp.gleam",
                TIMESTAMP_SOURCE,
            ),
            ModuleSource::new("main", "src/main.gleam", source),
        ]
    } else {
        vec![
            ModuleSource::new(
                "gleam/time/calendar",
                "src/gleam/time/calendar.gleam",
                if root_module == "gleam/time/calendar" {
                    source
                } else {
                    CALENDAR_SOURCE
                },
            ),
            ModuleSource::new(
                "gleam/time/timestamp",
                "src/gleam/time/timestamp.gleam",
                if root_module == "gleam/time/timestamp" {
                    source
                } else {
                    TIMESTAMP_SOURCE
                },
            ),
        ]
    };
    let providers = host_providers::<GleamTimeProfile<Source>>()
        .expect("synthetic Time providers should register");
    let hosts = HostProviderSet::with_providers(
        Vec::<HostModule<GleamTimeProfile<Source>>>::new(),
        providers,
    )
    .expect("synthetic Time provider modules should be unique");
    let typed = compile_typed_host_program(
        "gleam_time",
        root_module,
        [PackageSource::new(
            "gleam_time",
            Vec::<EcoString>::new(),
            modules,
        )],
        hosts,
    )
    .expect("synthetic Time source should compile");
    let plan = plan_host_program(typed).expect("synthetic Time source should plan");
    HostedExecution::try_from_module_plan(plan).expect("synthetic Time execution should seal")
}
