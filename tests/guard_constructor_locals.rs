use camino::Utf8PathBuf;
use geam::embedding::{BigInt, CallError, FunctionDeclaration, HostedModuleBuilder, StringValue};
use geam::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, IoStream, host_providers};
use geam::{HostProviderSet, HostedExecution, compile_typed_host_project, plan_host_program};

#[path = "support/execution_host.rs"]
mod execution_fixture;
#[path = "support/guard_constructor_fixture.rs"]
mod fixture;
#[path = "support/workspace_dependencies.rs"]
mod workspace_dependencies;

#[test]
fn runs_local_guards_multi_subject_patterns_and_complete_original_clip() {
    let directory = tempfile::tempdir().unwrap();
    fixture::copy_project(directory.path());
    let typed = compile_typed_host_project(
        Utf8PathBuf::from_path_buf(directory.path().to_path_buf()).unwrap(),
        "guard_constructor_locals",
        HostProviderSet::from_providers(host_providers::<GleamStdlibProfile>().unwrap()).unwrap(),
    )
    .expect("original package sources should compile");
    let mut execution = HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap())
        .expect("guard locals and original clip bodies should seal");
    let mut state = GleamStdlibRunState::from_seed([0; 32]);
    let mut echo = Vec::new();
    let value = execution_fixture::run(&mut execution, &mut state, &mut echo).unwrap();
    assert_eq!(value.inspect().to_string(), "Nil");
    assert!(echo.is_empty());
    assert_eq!(state.io_outputs().len(), 1);
    assert_eq!(state.io_outputs()[0].stream(), IoStream::Stdout);
    assert_eq!(state.io_outputs()[0].text().as_bytes(), fixture::OUTPUT);
}

#[test]
fn typed_embedding_reuses_generic_guards_captures_and_nested_remainder_bindings() {
    let typed = compile_typed_host_project(
        Utf8PathBuf::from_path_buf(fixture::project_root()).unwrap(),
        "guard_constructor_locals",
        HostProviderSet::from_providers(host_providers::<GleamStdlibProfile>().unwrap()).unwrap(),
    )
    .unwrap();
    let (mut bindings, main) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), ()>::new("main"))
        .unwrap();
    let integers = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
            "matches_int",
        ))
        .unwrap();
    let strings = bindings
        .function(FunctionDeclaration::<(StringValue, StringValue), bool>::new("matches_string"))
        .unwrap();
    let captured = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
            "captured_match",
        ))
        .unwrap();
    let remainder_bool = bindings
        .function(FunctionDeclaration::<(bool, bool), bool>::new(
            "remainder_bool",
        ))
        .unwrap();
    let remainder_custom = bindings
        .function(FunctionDeclaration::<(bool, bool), bool>::new(
            "remainder_custom",
        ))
        .unwrap();
    let arithmetic = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt, BigInt), bool>::new(
            "arithmetic_captured_match",
        ))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let host = execution_fixture::TestHost::default();
    let mut state = GleamStdlibRunState::from_seed([0; 32]);
    let mut echo = Vec::new();
    let actual = host
        .block_on(
            module.with_execution(&host, &mut state, &mut echo, async |scope| {
                scope.call(&main, ()).await?;
                let mut results = Vec::new();
                for (value, expected) in [(7, 7), (7, 8), (8, 8), (7, 7)] {
                    results.push(
                        scope
                            .call(&integers, (value.into(), expected.into()))
                            .await?,
                    );
                }
                for (value, expected) in [("ab", "ab"), ("ab", "ac")] {
                    results.push(
                        scope
                            .call(&strings, (value.into(), expected.into()))
                            .await?,
                    );
                }
                for (value, expected) in [(7, 7), (7, 8), (7, 7)] {
                    results.push(
                        scope
                            .call(&captured, (value.into(), expected.into()))
                            .await?,
                    );
                }
                for function in [&remainder_bool, &remainder_custom] {
                    for input in [(true, false), (false, false), (true, true), (true, false)] {
                        results.push(scope.call(function, input).await?);
                    }
                }
                for (value, offset, expected) in [
                    ("9223372036854775807", "1", "9223372036854775807"),
                    ("7", "170141183460469231731687303715884105727", "7"),
                    (
                        "170141183460469231731687303715884105727",
                        "1",
                        "170141183460469231731687303715884105727",
                    ),
                    (
                        "170141183460469231731687303715884105727",
                        "1",
                        "170141183460469231731687303715884105726",
                    ),
                    ("7", "-9223372036854775816", "7"),
                    ("7", "170141183460469231731687303715884105727", "7"),
                ] {
                    results.push(
                        scope
                            .call(
                                &arithmetic,
                                (
                                    value.parse().unwrap(),
                                    offset.parse().unwrap(),
                                    expected.parse().unwrap(),
                                ),
                            )
                            .await?,
                    );
                }
                Ok::<_, CallError>(results)
            }),
        )
        .unwrap()
        .unwrap();
    assert_eq!(
        actual,
        [
            true, false, true, true, true, false, true, false, true, true, false, false, true,
            true, false, false, true, true, true, true, false, true, true,
        ]
    );
    assert!(echo.is_empty());
    assert_eq!(state.io_outputs().len(), 1);
    assert_eq!(state.io_outputs()[0].text().as_bytes(), fixture::OUTPUT);
}
