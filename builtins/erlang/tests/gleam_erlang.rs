use camino::Utf8Path;
use geam_core::{HostProviderSet, HostedExecution, compile_typed_host_project, plan_host_program};
use geam_erlang::{Configuration, GleamErlangProfile, GleamErlangRunState, host_providers};
use geam_stdlib::GleamStdlibRunState;

#[path = "gleam_erlang/callbacks.rs"]
mod callbacks;
#[path = "../../../tests/support/execution_host.rs"]
mod execution_fixture;
#[path = "gleam_erlang/surface.rs"]
mod surface;
#[path = "../../../tests/support/workspace_dependencies.rs"]
mod workspace_dependencies;

fn run_fixture(module: &str) {
    run_clocked_fixture(module, &[]);
}

fn project_root() -> camino::Utf8PathBuf {
    let root = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/project");
    static PREPARED: std::sync::OnceLock<Result<(), String>> = std::sync::OnceLock::new();
    workspace_dependencies::prepare(
        &PREPARED,
        root.as_std_path(),
        "gleam",
        &["deps", "download"],
        "`gleam deps download`",
    );
    root
}

fn run_clocked_fixture(module: &str, pending_advances: &[u64]) {
    let root = project_root();
    let mut providers = geam_stdlib::host_providers::<GleamErlangProfile>().unwrap();
    providers.extend(host_providers::<GleamErlangProfile>().unwrap());
    let typed = compile_typed_host_project(
        &root,
        module,
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let resources = typed.package_resources().clone();
    let plan = plan_host_program(typed).unwrap();
    let mut execution = HostedExecution::try_from_module_plan(plan).unwrap();
    let mut state = GleamErlangRunState {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
    };
    let mut echo = Vec::new();
    let host = execution_fixture::TestHost::default();
    let actual = {
        let mut running = std::pin::pin!(execution.run_main(&host, &mut state, &mut echo));
        for milliseconds in pending_advances {
            assert!(host.poll(running.as_mut()).is_pending());
            host.advance(std::time::Duration::from_millis(*milliseconds));
        }
        let std::task::Poll::Ready(actual) = host.poll(running.as_mut()) else {
            panic!("{module} did not finish at its declared virtual deadline");
        };
        actual.unwrap()
    };
    let source =
        std::fs::read_to_string(root.join("src").join(module).with_extension("gleam")).unwrap();
    let expected = source
        .lines()
        .rev()
        .find_map(|line| line.strip_prefix("// @geam:expect "))
        .unwrap();
    assert_eq!(actual.inspect().to_string(), expected);
    assert!(echo.is_empty());
    assert!(state.stdlib.io_outputs().is_empty());
}

#[test]
fn named_and_unnamed_subjects_preserve_order_and_unmatched_mail() {
    run_fixture("subjects");
}

#[test]
fn bounded_mailbox_scans_preserve_queued_matches_at_zero_timeout() {
    run_clocked_fixture("mailbox_backlog", &[9, 1]);
}

#[test]
fn selector_callback_waits_and_resumes_in_the_receiving_process() {
    run_fixture("selector_wait");
}

#[test]
fn original_native_value_wrappers_and_record_selection() {
    run_fixture("native_values");
}

#[test]
fn monitor_delivery_and_name_cleanup_follow_process_termination() {
    run_fixture("lifecycle");
}

#[test]
fn linked_stateful_service_handles_repeated_request_reply_and_shutdown() {
    run_fixture("service");
}

#[test]
fn links_propagate_normal_and_abnormal_exits_and_unlink_suppresses_delivery() {
    run_fixture("links");
}

#[test]
fn normal_and_kill_exit_signals_distinguish_self_other_and_trapping_processes() {
    run_fixture("exit_signals");
}

#[test]
fn persistent_selector_operations_agree_with_their_native_dynamic_views() {
    run_fixture("selectors");
}

#[test]
fn timers_expire_at_the_host_deadline_and_cancel_exactly_once() {
    run_clocked_fixture("timers", &[9, 1]);
}

#[test]
fn timers_outlive_the_sender_cancel_with_pid_targets_and_resolve_names_at_delivery() {
    run_clocked_fixture("timer_lifetimes", &[9, 1]);
}

#[test]
fn receive_select_and_sleep_use_the_host_clock() {
    run_clocked_fixture("timeouts", &[9, 1, 9, 1, 9, 1]);
}

#[test]
fn sleep_accepts_large_integers_without_truncation_or_blocking() {
    run_clocked_fixture("long_sleep", &[u64::from(u32::MAX) - 1, 1, 6, 1]);
}

#[test]
fn invalid_timeouts_fail_before_receiving_an_already_queued_message() {
    run_fixture("invalid_timeouts");
}

#[test]
fn source_and_provider_failures_preserve_diagnostics_in_process_exit_messages() {
    run_fixture("failures");
}

#[test]
fn selected_callbacks_and_mapping_can_finish_after_the_receive_timeout() {
    run_clocked_fixture("selector_deadline", &[5, 14, 1, 19, 1]);
}

#[test]
fn every_selector_callback_family_can_wait_in_its_receiving_process() {
    run_fixture("selector_callbacks");
}

#[test]
fn resource_lookup_uses_exact_resolved_packages_not_directory_existence() {
    assert!(!project_root().join("priv").exists());
    run_fixture("resources");
}

#[test]
fn atoms_and_charlists_preserve_actual_native_values_through_dynamic() {
    run_fixture("native_interop");
}

#[test]
fn original_erlang_and_geam_pass_the_same_handshaken_public_compositions() {
    let output = std::process::Command::new("gleam")
        .args(["run", "--target", "erlang", "--module", "oracle"])
        .current_dir(project_root())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    run_fixture("oracle");
}
