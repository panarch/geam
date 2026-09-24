extern crate geam as geam_core;

#[path = "../../../../../../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam::{
    HostComponentProfile, HostProfile, HostProviderComponentInitialization,
    HostProviderComponentRegistration, HostProviderConfiguration, HostProviderSet, HostedExecution,
    ModuleSource, PackageSource, compile_typed_host_program, plan_host_program,
};
use geam_arguments_fixture::{Component, State, Stores};
use std::collections::BTreeMap;
use std::ffi::OsString;

struct Profile;

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &Stores {
        stores
    }

    fn component_state(state: &mut State) -> &mut State {
        state
    }
}

#[test]
fn initialization_captures_the_host_process_and_rejects_configuration() {
    let expected: Vec<_> = std::env::args_os().skip(1).collect();
    let state = Component::initialize(&HostProviderConfiguration::empty()).unwrap();
    assert_eq!(state.arguments, expected);
    let configuration =
        HostProviderConfiguration::new(BTreeMap::from([("unexpected".into(), true.into())]));
    let error = Component::initialize(&configuration).err().unwrap();
    assert_eq!(error.component_id(), "geam-arguments-fixture");
    assert_eq!(error.reason(), "provider does not accept configuration");
}

#[test]
fn ordinary_gleam_consumes_snapshots_from_caller_owned_host_state() {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let typed = compile_typed_host_program(
        "application_arguments",
        "main",
        [PackageSource::new(
            "application_arguments",
            Vec::<&str>::new(),
            [
                ModuleSource::new(
                    "application_arguments",
                    "src/application_arguments.gleam",
                    include_str!("../../../project/packages/application_arguments/src/application_arguments.gleam"),
                ),
                ModuleSource::new(
                    "main",
                    "src/main.gleam",
                    r#"import application_arguments
pub fn main() {
  let snapshot = application_arguments.snapshot()
  let expected = case snapshot.0 {
    [] -> []
    ["", "a b", "--help", "x=y"] -> [[], [97, 32, 98], [45, 45, 104, 101, 108, 112], [120, 61, 121]]
    ["한글", "a�"] -> PLATFORM_UNITS
    _ -> panic as "unexpected arguments"
  }
  assert snapshot.1 == expected
  snapshot.0
}
"#.replace("PLATFORM_UNITS", if cfg!(windows) {
                        "[[54620, 44544], [97, 55296]]"
                    } else {
                        "[[237, 149, 156, 234, 184, 128], [97, 255]]"
                    }),
                ),
            ],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let mut state = State {
        arguments: Vec::new(),
    };
    let empty = execution_fixture::run(&mut execution, &mut state, &mut Vec::new()).unwrap();
    assert_eq!(empty.inspect().to_string(), "[]");
    state.arguments = ["", "a b", "--help", "x=y"].map(OsString::from).into();
    let first = execution_fixture::run(&mut execution, &mut state, &mut Vec::new()).unwrap();
    assert_eq!(
        first.inspect().to_string(),
        "[\"\", \"a b\", \"--help\", \"x=y\"]"
    );
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        state.arguments = vec!["한글".into(), OsString::from_vec(vec![97, 255])];
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        state.arguments = vec!["한글".into(), OsString::from_wide(&[97, 0xd800])];
    }
    let native = execution_fixture::run(&mut execution, &mut state, &mut Vec::new()).unwrap();
    assert_eq!(native.inspect().to_string(), "[\"한글\", \"a�\"]");
    drop(state);
    drop(execution);
    assert_eq!(empty.inspect().to_string(), "[]");
    assert_eq!(
        first.inspect().to_string(),
        "[\"\", \"a b\", \"--help\", \"x=y\"]"
    );
}
