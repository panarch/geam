//! Original, pinned OTP source running through the provider's public component.
extern crate geam as geam_core;

#[path = "original_otp/cancellation.rs"]
mod cancellation;
#[path = "original_otp/support.rs"]
mod support;

use geam::Value;
use geam::execution::{ExecutionUnit, UnitExit};
use geam::gleam_stdlib::{GleamStdlibRunState, IoStream};
use support::{execution, execution_fixture};

#[test]
fn original_warning_boundary_rejects_foreign_character_storage_before_emitting_a_warning() {
    use geam::gleam_erlang::{Charlist, CharlistSchema, Component as ErlangComponent};
    use geam::host::{
        HostCall, HostCallCompletion, HostCallError, HostConstructions, HostListType,
        HostProviderComponentRegistration, HostProviderModule, HostProviderSet, HostStoredValue,
        HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    };
    use geam::{
        HostedExecution, ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
    };
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use support::Profile;

    type Constructions = HostTypeList<Charlist, HostTypeList<HostListType<char>, HostTypeListEnd>>;
    type Call<'call> = HostCall<'call, Profile, ErlangComponent<Profile>, Charlist>;
    type Completion<'call> = Result<HostCallCompletion<'call, Charlist>, HostCallError>;
    fn retained(
        previous: Arc<Mutex<Option<HostStoredValue<HostListType<char>>>>>,
    ) -> impl for<'call> Fn(Call<'call>, HostConstructions<'call, Constructions>) -> Completion<'call>
    {
        move |mut call, constructions| {
            let saved = previous.lock().unwrap().take();
            let construction = constructions.at::<HostTypeIndex0>();
            let value = match saved {
                Some(characters) => call.construct_external(construction, characters),
                None => {
                    let characters = call.construct_list(
                        constructions.at::<HostTypeIndexNext<HostTypeIndex0>>(),
                        "retained".chars(),
                    );
                    call.construct_external_with::<CharlistSchema, HostTypeListEnd>(
                        construction,
                        |builder| {
                            *previous.lock().unwrap() =
                                Some(builder.store::<HostListType<char>>(characters));
                            builder.store::<HostListType<char>>(characters)
                        },
                    )
                }
            };
            Ok(call.return_value(value))
        }
    }
    fn modules(root: &Path, directory: &Path, output: &mut Vec<ModuleSource>) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                modules(root, &path, output);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "gleam")
            {
                let name = path.strip_prefix(root).unwrap().with_extension("");
                let name = name.to_str().unwrap().replace('\\', "/");
                let mut source = std::fs::read_to_string(&path).unwrap();
                if name == "gleam/otp/actor" {
                    // The original private source body stays unchanged; this test
                    // entry calls its native boundary with host-supplied Charlist values.
                    source.push_str("\npub fn probe_warning(format: Charlist, arguments: List(Charlist)) { log_warning(format, arguments) }\n");
                }
                output.push(ModuleSource::new(name, path.to_str().unwrap(), source));
            }
        }
    }
    let (_, mut state) = execution("otp_service_fixture");
    let packages = Path::new(env!("CARGO_MANIFEST_DIR")).join("../project/build/packages");
    let mut dependencies = Vec::new();
    for (package, requires) in [
        ("gleam_stdlib", vec![]),
        ("gleam_erlang", vec!["gleam_stdlib"]),
        ("gleam_otp", vec!["gleam_stdlib", "gleam_erlang"]),
    ] {
        let root = packages.join(package).join("src");
        let mut sources = Vec::new();
        modules(&root, &root, &mut sources);
        dependencies.push(PackageSource::new(package, requires, sources));
    }
    let host = execution_fixture::TestHost::default();
    for warning in [
        "actor.probe_warning(retained(), [])",
        "actor.probe_warning(charlist.from_string(\"~s\"), [retained()])",
    ] {
        let previous = Arc::new(Mutex::new(None));
        for body in ["let _ = retained() Nil", warning] {
            let mut sources = dependencies.clone();
            sources.push(PackageSource::new(
                "application",
                ["gleam_stdlib", "gleam_erlang", "gleam_otp"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    format!(
                        r#"
import gleam/erlang/charlist
import gleam/otp/actor
@external(erlang, "host", "retained") fn retained() -> charlist.Charlist
pub fn main() {{ {body} }}
"#
                    ),
                )],
            ));
            let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
            providers.extend(geam::gleam_erlang::host_providers::<Profile>().unwrap());
            providers.extend(<geam_otp_service_fixture::Component<Profile> as HostProviderComponentRegistration<Profile>>::providers().unwrap().into_iter().filter(|module| module.package() == "gleam_otp"));
            providers.push(HostProviderModule::new("application", "main").unwrap()
                .with_scoped_function_and_constructions::<ErlangComponent<Profile>, (), Charlist, Constructions, _>("retained", retained(previous.clone())).unwrap());
            let typed = compile_typed_host_program(
                "application",
                "main",
                sources,
                HostProviderSet::from_providers(providers).unwrap(),
            )
            .unwrap();
            let mut execution =
                HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
            let mut echo = Vec::new();
            let result = host.block_on(execution.run_main(&host, &mut state, &mut echo));
            if body == warning {
                assert_eq!(
                    result.err().unwrap().to_string(),
                    "host function gleam_otp::gleam/otp/actor.log_warning failed: retained value belongs to another owner or source type"
                );
            } else {
                assert_eq!(result.unwrap().try_into_value().unwrap(), Value::Nil);
            }
            assert!(echo.is_empty());
            assert!(state.provider.warnings.is_empty());
        }
    }
}

#[test]
fn original_actor_system_callbacks_share_the_mailbox_and_receiver() {
    let (mut execution, mut state) = execution("otp_service_fixture");
    let host = execution_fixture::TestHost::default();
    for _ in 0..2 {
        state.stdlib = GleamStdlibRunState::from_seed([0; 32]);
        let mut echo = Vec::new();
        let result = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(result, Value::Nil);
        assert!(echo.is_empty());
        assert!(state.provider.warnings.is_empty());
        assert_eq!(
            state
                .stdlib
                .io_outputs()
                .iter()
                .map(|output| (output.stream(), output.text().to_string()))
                .collect::<Vec<_>>(),
            [
                (
                    IoStream::Stdout,
                    "original actor: state, suspend, resume\n".to_owned()
                ),
                (
                    IoStream::Stdout,
                    "original static child: retained callback in supervisor\n".to_owned()
                ),
                (
                    IoStream::Stdout,
                    "original factory: two typed callbacks, named and pid handles\n".to_owned()
                )
            ]
        );
    }
}

#[test]
fn mapped_static_child_preserves_its_capture_data_failure_and_supervisor_context() {
    let (mut execution, mut state) = execution("otp_mapped_child");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    assert_eq!(
        host.block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap(),
        Value::Nil,
    );
    assert!(echo.is_empty());
    assert!(state.stdlib.io_outputs().is_empty());
    let observed = state.observed.lock().unwrap();
    assert_eq!(observed.units.len(), 5);
    assert_eq!(observed.finished.len(), 5);
    assert_eq!(observed.closed, 1);
    assert!(observed.units.iter().all(|unit| !unit.is_active()));
}

#[test]
fn original_actor_failure_exit_and_unexpected_message_paths() {
    let (mut execution, mut state) = execution("otp_lifecycle");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    assert_eq!(
        host.block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap(),
        Value::Nil,
    );
    assert!(echo.is_empty());
    assert_eq!(
        state.provider.warnings,
        [
            "Actor discarding unexpected message: \"unexpected fixture message\"",
            "Actor discarding unexpected message: System(\"bad request\", Nil)",
            "Actor discarding unexpected message: Other(0, Nil)"
        ]
    );
    assert_eq!(
        state.stdlib.io_outputs()[0].text(),
        "original actor: failed init, exited init, normal and abnormal stop\n"
    );
}

#[test]
fn original_initialiser_times_out_only_after_the_host_deadline() {
    let (mut execution, mut state) = execution("otp_init_timeout");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let run = execution.run_main(&host, &mut state, &mut echo);
    let mut run = std::pin::pin!(run);
    assert!(host.poll(run.as_mut()).is_pending());
    host.advance(std::time::Duration::from_millis(4));
    assert!(host.poll(run.as_mut()).is_pending());
    host.advance(std::time::Duration::from_millis(1));
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
}

#[test]
fn original_supervisor_errors_restart_budget_and_graceful_shutdown() {
    let (mut execution, mut state) = execution("otp_supervision");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let run = execution.run_main(&host, &mut state, &mut echo);
    let mut run = std::pin::pin!(run);
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
}

#[test]
fn original_factory_requests_report_missing_names_and_exited_supervisors() {
    let (mut execution, mut state) = execution("otp_factory_request_errors");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
    drop(run);
    let failures = observed
        .lock()
        .unwrap()
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Host(error)) => Some((
                error.function().to_string(),
                error.failure().message().to_string(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        failures,
        [
            (
                "erlang_count_children".into(),
                "factory supervisor name is not registered".into()
            ),
            (
                "erlang_start_child".into(),
                "factory supervisor name is not registered".into()
            ),
            (
                "erlang_count_children".into(),
                "factory supervisor has exited".into()
            ),
            (
                "erlang_start_child".into(),
                "factory supervisor has exited".into()
            ),
        ]
    );
}

#[test]
fn original_supervisors_stop_siblings_after_restart_failure_and_static_budget_exhaustion() {
    let (mut execution, mut state) = execution("otp_restart_failures");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
}

#[test]
fn malformed_mailbox_requests_fail_before_invoking_retained_callbacks() {
    let (mut execution, mut state) = execution("otp_native_messages");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
    drop(run);
    let failures = observed
        .lock()
        .unwrap()
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Host(error)) => {
                Some(error.failure().message().to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        failures,
        [
            "factory argument does not match the retained callback",
            "factory message has no source Pid",
            "factory message has no source Pid",
            "unknown factory operation",
            "factory message is missing a field",
            "exit message has no source Pid",
        ]
    );
}

#[test]
fn restarting_callbacks_propagate_source_panics_and_release_linked_siblings() {
    let (mut execution, mut state) = execution("otp_restart_panics");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    assert_eq!(
        host.block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap(),
        Value::Nil
    );
    let panics = observed
        .lock()
        .unwrap()
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Panic(panic)) => {
                Some((panic.kind(), panic.message().clone()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        panics,
        [
            (
                geam::PanicKind::Panic,
                geam::PanicMessage::Explicit("static restart callback".into())
            ),
            (
                geam::PanicKind::Panic,
                geam::PanicMessage::Explicit("factory restart callback".into())
            ),
        ]
    );
}

#[test]
fn factory_count_rejects_a_non_integer_response_from_a_registered_peer() {
    let (mut execution, mut state) = execution("otp_factory_count_reply");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    assert_eq!(
        host.block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap(),
        Value::Nil
    );
    let failures = observed
        .lock()
        .unwrap()
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Host(error)) => {
                Some(error.failure().message().to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(failures, ["factory count response is not an integer"]);
}

#[test]
fn failed_startup_cleanup_propagates_clock_overflow_and_releases_started_children() {
    use geam::execution::ExecutionHost;
    use std::time::Duration;

    let (mut execution, mut state) = execution("otp_startup_clock_range");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert!(host.poll(run.as_mut()).is_pending());
    for bit in (0..64).rev() {
        let step = Duration::from_secs(1 << bit);
        if host.now().checked_add(step).is_some() {
            host.advance(step);
        }
    }
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
    drop(run);
    let failures = observed
        .lock()
        .unwrap()
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Host(error)) => {
                Some(error.failure().message().to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(failures, ["shutdown timeout exceeds the host clock range"]);
}

#[test]
fn original_supervisor_policies_callbacks_and_parent_exit_keep_their_failure_boundaries() {
    let (mut execution, mut state) = execution("otp_supervisor_edges");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
    drop(run);
    let observation = observed.lock().unwrap();
    let host_failures = observation
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Host(error)) => {
                Some(error.failure().message().to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        host_failures,
        [
            "fixture supports OneForOne with Never auto shutdown",
            "restart intensity must be nonnegative",
            "restart period must be positive seconds",
        ]
    );
    let panics = observation
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Panic(panic)) => {
                Some((panic.kind(), panic.message().clone()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        panics,
        [
            (
                geam::PanicKind::Panic,
                geam::PanicMessage::Explicit("static start callback".into())
            ),
            (
                geam::PanicKind::Panic,
                geam::PanicMessage::Explicit("factory start callback".into())
            ),
        ]
    );
}

#[test]
fn original_supervisor_kills_a_child_that_ignores_shutdown() {
    let (mut execution, mut state) = execution("otp_shutdown_timeout");
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let run = execution.run_main(&host, &mut state, &mut echo);
    let mut run = std::pin::pin!(run);
    assert!(host.poll(run.as_mut()).is_pending());
    host.advance(std::time::Duration::from_millis(4));
    assert!(host.poll(run.as_mut()).is_pending());
    host.advance(std::time::Duration::from_millis(1));
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
}

#[test]
fn shutdown_propagates_clock_overflow_through_both_supervisors() {
    use geam::execution::ExecutionHost;
    use std::time::Duration;

    let (mut execution, mut state) = execution("otp_shutdown_clock_range");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    let mut run = Box::pin(execution.run_main(&host, &mut state, &mut echo));
    assert!(host.poll(run.as_mut()).is_pending());
    for bit in (0..64).rev() {
        let step = Duration::from_secs(1 << bit);
        if host.now().checked_add(step).is_some() {
            host.advance(step);
        }
    }
    assert_eq!(
        host.poll(run.as_mut())
            .map(|result| result.unwrap().try_into_value().unwrap()),
        std::task::Poll::Ready(Value::Nil)
    );
    drop(run);
    let failures = observed
        .lock()
        .unwrap()
        .exits
        .iter()
        .filter_map(|exit| match exit {
            UnitExit::Failed(geam::ExecutionError::Host(error)) => {
                Some(error.failure().message().to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        failures,
        ["shutdown timeout exceeds the host clock range"; 2]
    );
}

#[test]
fn cancelling_the_domain_releases_otp_waits_children_and_retained_callback_captures() {
    let (mut execution, mut state) = execution("otp_lifetime");
    let observed = std::sync::Arc::clone(&state.observed);
    let host = execution_fixture::TestHost::default();
    let mut echo = Vec::new();
    {
        let run = execution.run_main(&host, &mut state, &mut echo);
        let mut run = std::pin::pin!(run);
        assert!(host.poll(run.as_mut()).is_pending());
        let observation = observed.lock().unwrap();
        assert_eq!(observation.units.len(), 4);
        assert_eq!(observation.closed, 0);
        assert_eq!(observation.dropped_captures, 0);
        assert!(observation.units.iter().all(ExecutionUnit::is_active));
    }
    host.step();
    let observation = observed.lock().unwrap();
    assert!(observation.units.iter().all(|unit| !unit.is_active()));
    assert_eq!(observation.closed, 1);
    assert_eq!(observation.dropped_captures, 1);
    assert_eq!(observation.finished.len(), 4);
}
