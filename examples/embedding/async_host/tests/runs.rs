use std::process::Command;

#[path = "../src/geam_bindings.rs"]
mod geam_bindings;

#[test]
fn file_failures_remain_source_results_and_do_not_change_the_direct_call() {
    use geam::embedding::{WorkModuleBuilder, with_execution_scope};
    let directory = tempfile::tempdir().expect("file owner");
    let path = directory.path().join("missing.txt");
    let program = geam_bindings::project().compile().expect("project");
    let (bindings, functions) =
        geam_bindings::bind(WorkModuleBuilder::new(program).expect("plan")).expect("bindings");
    let mut module = bindings.seal().expect("sealed module");
    let mut state = geam_bindings::RunStateInputs {
        example_async_files: geam::HostProviderConfiguration::empty(),
    }
    .initialize()
    .expect("provider state");
    let mut echo = |value: geam::EchoOutput| panic!("unexpected Echo: {value}");
    futures::executor::block_on(with_execution_scope(async |guard| {
        let mut scope = module.attach(guard, &mut state, &mut echo);
        let work = scope
            .call(
                &functions.greeting,
                (path.to_str().expect("UTF-8 path").into(),),
            )
            .expect("work");
        let failed = scope
            .observe(&work)
            .await
            .expect("source Error is an ordinary completion");
        failed.read(|result| assert!(!result.expect_err("missing file").is_empty()));
        std::fs::write(&path, [0xff]).expect("non-UTF-8 file");
        let work = scope
            .call(
                &functions.greeting,
                (path.to_str().expect("UTF-8 path").into(),),
            )
            .expect("independent work");
        scope
            .observe(&work)
            .await
            .expect("ordinary completion")
            .read(|result| assert!(result.is_err()));
        assert_eq!(
            scope
                .call(&functions.double, (21.into(),))
                .expect("unrelated direct entry"),
            42.into()
        );
    }));
}

#[test]
fn constructs_and_drives_shared_work_without_changing_an_ordinary_call() {
    let output = Command::new(env!("CARGO_BIN_EXE_geam-rust-embedding-async-host"))
        .output()
        .expect("the async-host embedding example should run");

    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        output.stdout,
        b"double: 42\ncreated\nHello from Rust\nagain: Hello from Rust\n"
    );
    assert_eq!(output.stderr, b"");
}
