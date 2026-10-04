extern crate geam as geam_core;

#[path = "../../../../support/execution_host.rs"]
mod execution_fixture;

#[path = "support/profile.rs"]
mod profile;

use geam::gleam_erlang::Configuration;
use geam::gleam_stdlib::GleamStdlibRunState;
use geam::{HostedExecution, Value, compile_typed_host_project, plan_host_program};
use geam_selective_receive_service_fixture::Component;
use profile::{Profile, State, providers};

#[test]
fn original_references_are_selected_without_requeuing_other_messages() {
    let project = camino::Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let lock = std::fs::read(project.join("manifest.toml")).unwrap();
    let acquired = std::process::Command::new("gleam")
        .args(["deps", "download"])
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        acquired.status.success(),
        "{}",
        String::from_utf8_lossy(&acquired.stderr)
    );
    assert_eq!(std::fs::read(project.join("manifest.toml")).unwrap(), lock);
    let typed =
        compile_typed_host_project(&project, "selective_receive_service_fixture", providers())
            .unwrap();
    let resources = typed.package_resources().clone();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    let host = execution_fixture::TestHost::default();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
        provider: (),
    };
    let mut echo = Vec::new();
    for _ in 0..2 {
        let value = host
            .block_on(execution.run_main(&host, &mut state, &mut echo))
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(value, Value::Nil);
    }
    assert!(echo.is_empty());
    assert_eq!(
        state
            .stdlib
            .io_outputs()
            .iter()
            .map(|output| output.text())
            .collect::<Vec<_>>(),
        ["caller identity and mixed tags preserve mailbox order\n"; 2]
    );
    drop(execution);
    let typed =
        compile_typed_host_project(&project, "selective_receive_service_fixture", providers())
            .unwrap();
    let (builder, inspect) = geam::embedding::HostedModuleBuilder::<Profile>::new(typed)
        .unwrap()
        .function(geam::embedding::FunctionDeclaration::<(), ()>::new(
            "inspect_key",
        ))
        .unwrap();
    let mut module = builder.seal().unwrap();
    host.block_on(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            scope.call(&inspect, ()).await
        }),
    )
    .unwrap()
    .try_into_value()
    .unwrap()
    .unwrap();
    drop(module);
    drop(state);
    assert_eq!(echo.len(), 2);
    assert_eq!(
        echo[0].value().inspect().to_string(),
        echo[1].value().inspect().to_string()
    );
}
