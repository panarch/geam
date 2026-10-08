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

#[test]
fn exported_provider_callables_keep_original_keys_and_mailbox_identity() {
    use geam::embedding::{
        BigInt, CallableType, ExternalType, FunctionDeclaration, HostedModuleBuilder,
        NamedTypeSchema, StringValue,
    };

    struct KeySchema;
    impl NamedTypeSchema for KeySchema {
        const PACKAGE: &'static str = "selective_receive_service_fixture";
        const MODULE: &'static str = "selective_receive_service_fixture/native";
        const NAME: &'static str = "Key";
    }
    struct ReferenceSchema;
    impl NamedTypeSchema for ReferenceSchema {
        const PACKAGE: &'static str = "gleam_erlang";
        const MODULE: &'static str = "gleam/erlang/reference";
        const NAME: &'static str = "Reference";
    }
    struct PidSchema;
    impl NamedTypeSchema for PidSchema {
        const PACKAGE: &'static str = "gleam_erlang";
        const MODULE: &'static str = "gleam/erlang/process";
        const NAME: &'static str = "Pid";
    }
    type Key = ExternalType<KeySchema>;
    type Reference = ExternalType<ReferenceSchema>;
    type Pid = ExternalType<PidSchema>;
    type Message = (StringValue, Reference, BigInt);

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
    let (mut builder, key_callback) = HostedModuleBuilder::<Profile>::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), CallableType<(Reference,), Key>>::new("key_callback"))
        .unwrap();
    let send_callback = builder
        .function(FunctionDeclaration::<(), CallableType<(Pid, Message), ()>>::new("send_callback"))
        .unwrap();
    let current_process = builder
        .function(FunctionDeclaration::<(), Pid>::new("current_process"))
        .unwrap();
    let new_reference = builder
        .function(FunctionDeclaration::<(), Reference>::new("new_reference"))
        .unwrap();
    let equal = builder
        .function(FunctionDeclaration::<(Key, Key), bool>::new("keys_equal"))
        .unwrap();
    let next = builder
        .function(FunctionDeclaration::<(), Result<Message, ()>>::new(
            "next_message",
        ))
        .unwrap();
    let roundtrip = builder
        .function(FunctionDeclaration::<(), Result<Message, ()>>::new(
            "callable_mailbox_roundtrip",
        ))
        .unwrap();
    let mut module = builder.seal().unwrap();
    let host = execution_fixture::TestHost::default();
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        erlang: Configuration { resources },
        provider: (),
    };
    let mut echo = Vec::new();
    host.block_on(
        module.with_execution(&host, &mut state, &mut echo, async |scope| {
            let identity = scope.call(&new_reference, ()).await.unwrap();
            let other = scope.call(&new_reference, ()).await.unwrap();
            let make_key = scope.call(&key_callback, ()).await.unwrap();
            let key = scope.invoke(&make_key, (&identity,)).await.unwrap();
            let alias = scope.invoke(&make_key, (&identity,)).await.unwrap();
            let distinct = scope.invoke(&make_key, (&other,)).await.unwrap();
            assert!(scope.call(&equal, (&key, &alias)).await.unwrap());
            assert!(!scope.call(&equal, (&key, &distinct)).await.unwrap());
            let pid = scope.call(&current_process, ()).await.unwrap();
            let send = scope.call(&send_callback, ()).await.unwrap();
            scope
                .invoke(&send, (&pid, ("tcp".into(), &identity, BigInt::from(42))))
                .await
                .unwrap();
            // The entry that returned this Pid has completed. Sending to it is
            // a no-op; the following source entry has its own empty mailbox.
            assert!(scope.call(&next, ()).await.unwrap().is_err());
            let (tag, selected, payload) = scope.call(&roundtrip, ()).await.unwrap().unwrap();
            assert_eq!(tag.as_str().unwrap(), "tcp");
            assert_eq!(payload, BigInt::from(42));
            let selected_key = scope.invoke(&make_key, (&selected,)).await.unwrap();
            let selected_alias = scope.invoke(&make_key, (&selected,)).await.unwrap();
            assert!(
                scope
                    .call(&equal, (&selected_key, &selected_alias))
                    .await
                    .unwrap()
            );
            assert!(!scope.call(&equal, (&key, &selected_key)).await.unwrap());
            assert!(scope.call(&next, ()).await.unwrap().is_err());
        }),
    )
    .unwrap()
    .try_into_value()
    .unwrap();
    assert!(echo.is_empty());
}
