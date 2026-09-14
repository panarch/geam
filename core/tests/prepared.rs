use geam_core::__prepared_support as data;
use geam_core::embedding::{BigInt, CallError, FunctionDeclaration, ModuleBuilder};

static ARITHMETIC: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/arithmetic.rs");

static VALUES: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/values.rs");

#[path = "support/work_fixture.rs"]
mod work_fixture;
#[path = "fixtures/prepared/work_provider.rs"]
mod work_provider;

static WORK: data::HostedModuleArtifact = include!("fixtures/prepared/work.rs");

static ENTRY: data::HostedEntryArtifact = include!("fixtures/prepared/entry.rs");
static ENTRY_WORK: data::HostedEntryArtifact = include!("fixtures/prepared/entry_work.rs");
static ENTRY_FAILURE: data::HostedEntryArtifact = include!("fixtures/prepared/entry_failure.rs");

#[test]
fn standalone_artifacts_match_preparation_and_link_without_embedding_exports() {
    for (module, artifact, expected) in [
        ("entry", &ENTRY, include_str!("fixtures/prepared/entry.rs")),
        (
            "entry_work",
            &ENTRY_WORK,
            include_str!("fixtures/prepared/entry_work.rs"),
        ),
        (
            "entry_failure",
            &ENTRY_FAILURE,
            include_str!("fixtures/prepared/entry_failure.rs"),
        ),
    ] {
        assert_eq!(
            work_provider::prepare_entry(module).emit_rust(),
            expected.trim()
        );
        artifact.load(work_provider::hosts()).unwrap();
    }
}

#[cfg(feature = "tokio")]
#[test]
fn standalone_entries_preserve_generic_function_outer_work_and_source_failure_behavior() {
    use geam_core::execution::{RunError, TokioHost};
    use miette::Diagnostic;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for (artifact, expected) in [
        (&ENTRY, vec!["src/entry.gleam:2\n42"]),
        (
            &ENTRY_WORK,
            vec![
                "src/entry_work.gleam:4\n\"main\"",
                "src/entry_work.gleam:6\n42",
            ],
        ),
    ] {
        for _ in 0..2 {
            let mut entry = artifact.load(work_provider::hosts()).unwrap();
            let mut echo = Vec::new();
            runtime
                .block_on(entry.run(&host, &mut (), &mut echo))
                .unwrap();
            assert_eq!(
                echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
                expected
            );
        }
    }
    let mut entry = ENTRY_FAILURE.load(work_provider::hosts()).unwrap();
    let mut echo = Vec::new();
    let RunError::Execution(geam_core::ExecutionError::Panic(panic)) = runtime
        .block_on(entry.run(&host, &mut (), &mut echo))
        .unwrap_err()
    else {
        panic!("expected source panic from prepared main")
    };
    assert_eq!(panic.to_string(), "panic: prepared main failed");
    assert_eq!(panic.site().module(), "entry_failure");
    assert_eq!(panic.site().function(), "main");
    assert_eq!(
        echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
        ["src/entry_failure.gleam:2\n\"before failure\""]
    );
    let span = panic.site().span();
    let code = panic
        .source_code()
        .unwrap()
        .read_span(&(span.start()..span.end()).into(), 0, 0)
        .unwrap();
    assert_eq!(code.name(), Some("src/entry_failure.gleam"));
    assert!(
        std::str::from_utf8(code.data())
            .unwrap()
            .contains("prepared main failed")
    );
}

#[test]
fn incompatible_format_never_produces_a_prepared_binding_owner() {
    static INCOMPATIBLE: data::ModuleArtifact<std::convert::Infallible> = data::ModuleArtifact {
        format: 0,
        ..include!("fixtures/prepared/arithmetic.rs")
    };
    let error = INCOMPATIBLE.load().err().unwrap();
    assert_eq!(
        error.to_string(),
        "prepared format 0 is incompatible with format 1; regenerate the prepared program"
    );
}

#[test]
fn work_data_matches_preparation_and_contains_external_storage_families() {
    assert_eq!(
        work_provider::prepare().emit_rust(),
        include_str!("fixtures/prepared/work.rs").trim()
    );
    WORK.load(work_provider::hosts()).unwrap();
    let tables = &WORK.module.program.functions;
    for length in [
        tables.value_returns.external_functions.len(),
        tables.list_returns.external_list_functions.len(),
        tables.function_returns.external_function_functions.len(),
        tables
            .function_returns
            .external_list_function_functions
            .len(),
    ] {
        assert_ne!(length, 0);
    }
}

#[cfg(feature = "tokio")]
#[test]
fn emitted_work_retains_captures_shared_completion_and_scope_ownership() {
    use geam_core::embedding::{List, ObservationError};
    use work_fixture::WorkType;
    let mut bindings = WORK.load(work_provider::hosts()).unwrap();
    let make = bindings
        .function(FunctionDeclaration::<(BigInt,), WorkType<BigInt>>::new(
            "make",
        ))
        .unwrap();
    let collect = bindings
        .function(FunctionDeclaration::<(BigInt,), WorkType<List<BigInt>>>::new("collect"))
        .unwrap();
    let keep = bindings
        .function(FunctionDeclaration::<(WorkType<BigInt>,), WorkType<BigInt>>::new("keep"))
        .unwrap();
    let failure = bindings
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("failure"))
        .unwrap();
    let mut module = bindings.seal();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    let mut echo = Vec::new();
    let completed = runtime
        .block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                let work = scope.call(&make, (40.into(),)).await.unwrap();
                let same = scope.call(&keep, (&work,)).await.unwrap();
                let completed = scope.observe(&work).await.unwrap();
                let alias = scope.observe(&same).await.unwrap();
                completed.read(|left| alias.read(|right| assert!(std::ptr::eq(left, right))));
                assert_eq!(completed.read(Clone::clone), BigInt::from(42));
                let collected = scope.call(&collect, (40.into(),)).await.unwrap();
                let collected = scope.observe(&collected).await.unwrap();
                collected.read(|list| {
                    assert_eq!(list.len(), 2);
                    assert_eq!(list.read_item(0, Clone::clone), Some(BigInt::from(42)));
                    assert_eq!(list.read_item(1, Clone::clone), Some(BigInt::from(42)));
                });
                let failed = scope.call(&failure, ()).await.unwrap();
                let (
                    Err(ObservationError::Execution(first)),
                    Err(ObservationError::Execution(second)),
                ) = (scope.observe(&failed).await, scope.observe(&failed).await)
                else {
                    panic!("expected shared Gleam failure")
                };
                first.read(|left| {
                    second.read(|right| {
                        assert!(std::ptr::eq(left, right));
                        assert_eq!(left.to_string(), "panic: prepared work failed");
                    })
                });
                let _unobserved = scope.call(&make, (7.into(),)).await.unwrap();
                completed
            }),
        )
        .unwrap();
    assert_eq!(completed.read(Clone::clone), BigInt::from(42));
    let mut other = WORK.load(work_provider::hosts()).unwrap().seal();
    runtime
        .block_on(
            other.with_execution(&host, &mut (), &mut echo, async |scope| {
                assert!(matches!(
                    scope.call(&make, (40.into(),)).await,
                    Err(CallError::ForeignFunction)
                ));
            }),
        )
        .unwrap();
    assert!(echo.is_empty());
}

#[test]
fn emitted_program_preserves_value_closure_constant_and_failure_paths() {
    use geam_core::{ExecutionError, PanicDetails, PanicKind, PanicMessage, Value};
    use miette::Diagnostic;
    let mut bindings = VALUES.load().unwrap();
    let run = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .unwrap();
    let fail = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("fail"))
        .unwrap();
    let assertion = bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("assertion"))
        .unwrap();
    let module = bindings.seal();
    let mut echoes = Vec::new();
    assert_eq!(
        module.call(&run, (), &mut |output: geam_core::EchoOutput| echoes
            .push(output.to_string())),
        Ok(BigInt::from(42))
    );
    assert_eq!(echoes, ["src/example.gleam:150\n42"]);
    let CallError::Execution(ExecutionError::Panic(panic)) = module
        .call(&fail, (), &mut Vec::new())
        .unwrap_err()
        .into_materialized()
    else {
        panic!("expected Gleam panic")
    };
    assert_eq!(panic.kind(), PanicKind::Panic);
    assert_eq!(
        panic.message(),
        &PanicMessage::Explicit("prepared stop".into())
    );
    assert_eq!(panic.site().module(), "example");
    assert_eq!(panic.site().function(), "stop");
    let span = panic.site().span();
    let code = panic
        .source_code()
        .unwrap()
        .read_span(&(span.start()..span.end()).into(), 0, 0)
        .unwrap();
    assert_eq!(code.name(), Some("src/example.gleam"));
    assert!(
        std::str::from_utf8(code.data())
            .unwrap()
            .contains("prepared stop")
    );

    let CallError::Execution(ExecutionError::Panic(panic)) = module
        .call(&assertion, (7.into(),), &mut Vec::new())
        .unwrap_err()
        .into_materialized()
    else {
        panic!("expected let assert failure")
    };
    assert_eq!(panic.kind(), PanicKind::LetAssert);
    assert!(
        matches!(panic.details(), Some(PanicDetails::LetAssert { value: Value::Int(value), .. }) if value == &BigInt::from(7))
    );
    assert_eq!(
        module.call(&assertion, (42.into(),), &mut Vec::new()),
        Ok(BigInt::from(42))
    );
}

#[test]
fn selected_program_covers_all_plain_function_storage_families() {
    let tables = &VALUES.program.functions;
    assert_eq!(
        [
            tables.value_returns.external_functions.len(),
            tables.list_returns.external_list_functions.len(),
            tables.function_returns.external_function_functions.len(),
            tables
                .function_returns
                .external_list_function_functions
                .len(),
        ],
        [0; 4]
    );
    for (family, length) in [
        (
            "never_functions",
            tables.value_returns.never_functions.len(),
        ),
        ("int_functions", tables.value_returns.int_functions.len()),
        (
            "float_functions",
            tables.value_returns.float_functions.len(),
        ),
        (
            "string_functions",
            tables.value_returns.string_functions.len(),
        ),
        (
            "bit_array_functions",
            tables.value_returns.bit_array_functions.len(),
        ),
        (
            "utf_codepoint_functions",
            tables.value_returns.utf_codepoint_functions.len(),
        ),
        (
            "custom_functions",
            tables.value_returns.custom_functions.len(),
        ),
        ("bool_functions", tables.value_returns.bool_functions.len()),
        ("nil_functions", tables.value_returns.nil_functions.len()),
        (
            "tuple_functions",
            tables.value_returns.tuple_functions.len(),
        ),
        (
            "parameter_list_functions",
            tables.list_returns.parameter_list_functions.len(),
        ),
        (
            "int_list_functions",
            tables.list_returns.int_list_functions.len(),
        ),
        (
            "string_list_functions",
            tables.list_returns.string_list_functions.len(),
        ),
        (
            "bit_array_list_functions",
            tables.list_returns.bit_array_list_functions.len(),
        ),
        (
            "utf_codepoint_list_functions",
            tables.list_returns.utf_codepoint_list_functions.len(),
        ),
        (
            "custom_list_functions",
            tables.list_returns.custom_list_functions.len(),
        ),
        (
            "float_list_functions",
            tables.list_returns.float_list_functions.len(),
        ),
        (
            "bool_list_functions",
            tables.list_returns.bool_list_functions.len(),
        ),
        (
            "nil_list_functions",
            tables.list_returns.nil_list_functions.len(),
        ),
        (
            "tuple_list_functions",
            tables.list_returns.tuple_list_functions.len(),
        ),
        (
            "parameter_list_list_functions",
            tables.list_returns.parameter_list_list_functions.len(),
        ),
        (
            "list_list_functions",
            tables.list_returns.list_list_functions.len(),
        ),
        (
            "function_list_functions",
            tables.list_returns.function_list_functions.len(),
        ),
        (
            "int_function_functions",
            tables.function_returns.int_function_functions.len(),
        ),
        (
            "float_function_functions",
            tables.function_returns.float_function_functions.len(),
        ),
        (
            "string_function_functions",
            tables.function_returns.string_function_functions.len(),
        ),
        (
            "bit_array_function_functions",
            tables.function_returns.bit_array_function_functions.len(),
        ),
        (
            "utf_codepoint_function_functions",
            tables
                .function_returns
                .utf_codepoint_function_functions
                .len(),
        ),
        (
            "custom_function_functions",
            tables.function_returns.custom_function_functions.len(),
        ),
        (
            "bool_function_functions",
            tables.function_returns.bool_function_functions.len(),
        ),
        (
            "nil_function_functions",
            tables.function_returns.nil_function_functions.len(),
        ),
        (
            "tuple_function_functions",
            tables.function_returns.tuple_function_functions.len(),
        ),
        (
            "generic_function_functions",
            tables.function_returns.generic_function_functions.len(),
        ),
        (
            "never_function_functions",
            tables.function_returns.never_function_functions.len(),
        ),
        (
            "parameter_list_function_functions",
            tables
                .function_returns
                .parameter_list_function_functions
                .len(),
        ),
        (
            "parameter_list_list_function_functions",
            tables
                .function_returns
                .parameter_list_list_function_functions
                .len(),
        ),
        (
            "int_list_function_functions",
            tables.function_returns.int_list_function_functions.len(),
        ),
        (
            "string_list_function_functions",
            tables.function_returns.string_list_function_functions.len(),
        ),
        (
            "bit_array_list_function_functions",
            tables
                .function_returns
                .bit_array_list_function_functions
                .len(),
        ),
        (
            "utf_codepoint_list_function_functions",
            tables
                .function_returns
                .utf_codepoint_list_function_functions
                .len(),
        ),
        (
            "custom_list_function_functions",
            tables.function_returns.custom_list_function_functions.len(),
        ),
        (
            "float_list_function_functions",
            tables.function_returns.float_list_function_functions.len(),
        ),
        (
            "bool_list_function_functions",
            tables.function_returns.bool_list_function_functions.len(),
        ),
        (
            "nil_list_function_functions",
            tables.function_returns.nil_list_function_functions.len(),
        ),
        (
            "tuple_list_function_functions",
            tables.function_returns.tuple_list_function_functions.len(),
        ),
        (
            "list_list_function_functions",
            tables.function_returns.list_list_function_functions.len(),
        ),
        (
            "function_list_function_functions",
            tables
                .function_returns
                .function_list_function_functions
                .len(),
        ),
        (
            "function_function_functions",
            tables.function_returns.function_function_functions.len(),
        ),
    ] {
        assert_ne!(length, 0, "missing family {family}");
    }
}

#[test]
fn value_data_matches_complete_selected_preparation_output() {
    let module = geam_core::compile_typed_program(
        "example",
        [geam_core::ModuleSource::new(
            "example",
            "src/example.gleam",
            include_str!("fixtures/prepared/values.gleam"),
        )],
    )
    .unwrap();
    let (mut bindings, _) = ModuleBuilder::from_program(module)
        .unwrap()
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), BigInt>::new("fail"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("assertion"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/values.rs").trim()
    );
}

#[test]
fn plain_selection_failure_preserves_the_name_and_success_reserves_it() {
    let mut bindings = ARITHMETIC.load().unwrap();
    let missing = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("missing"))
        .err()
        .unwrap();
    assert_eq!(
        missing.to_string(),
        "function missing does not exist in the Gleam module"
    );
    let mismatch = bindings
        .function(FunctionDeclaration::<(), bool>::new("main"))
        .err()
        .unwrap();
    assert_eq!(
        mismatch.to_string(),
        "function main has type FunctionType { arguments: [], return_: Int }, expected FunctionType { arguments: [], return_: Bool }"
    );
    let main = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("main"))
        .unwrap();
    let duplicate = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("main"))
        .err()
        .unwrap();
    assert_eq!(
        duplicate.to_string(),
        "function main was selected more than once"
    );
    let module = bindings.seal();
    assert_eq!(
        module.call(&main, (), &mut Vec::new()),
        Ok(BigInt::from(42))
    );
}

#[test]
fn emitted_plain_program_loads_into_independent_callable_modules() {
    let mut first = ARITHMETIC.load().unwrap();
    let first_function = first
        .function(FunctionDeclaration::<(), BigInt>::new("main"))
        .unwrap();
    let first = first.seal();
    let mut second = ARITHMETIC.load().unwrap();
    let second_function = second
        .function(FunctionDeclaration::<(), BigInt>::new("main"))
        .unwrap();
    let second = second.seal();
    for _ in 0..3 {
        assert_eq!(
            first.call(&first_function, (), &mut Vec::new()).unwrap(),
            BigInt::from(42)
        );
        assert_eq!(
            second.call(&second_function, (), &mut Vec::new()).unwrap(),
            BigInt::from(42)
        );
        assert!(matches!(
            second.call(&first_function, (), &mut Vec::new()),
            Err(CallError::ForeignFunction)
        ));
    }
}

#[test]
fn plain_data_matches_preparation_output() {
    let module =
        geam_core::compile_typed_module("example", "src/example.gleam", "pub fn main() { 21 * 2 }")
            .unwrap();
    let (bindings, _) = ModuleBuilder::new(module)
        .unwrap()
        .function(FunctionDeclaration::<(), BigInt>::new("main"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/arithmetic.rs").trim()
    );
}

#[path = "fixtures/prepared/native_provider.rs"]
mod native_provider;

static NATIVE: data::HostedModuleArtifact = include!("fixtures/prepared/native.rs");

#[test]
fn hosted_loading_rejects_missing_providers_before_selecting_functions() {
    let providers = geam_core::HostProviderSet::<geam_core::StatelessHostProfile>::new([]).unwrap();
    let error = NATIVE.load(providers).err().unwrap();
    assert_eq!(
        error.to_string(),
        "prepared provider registration mismatch: Registration { package: \"application\", module: \"main\", function: \"equal_native\", reason: Missing }; regenerate with the matching providers"
    );
}

#[test]
fn hosted_selection_failure_preserves_the_name_and_success_reserves_it() {
    use geam_core::embedding::BindingError;
    use std::error::Error;

    let mut bindings = NATIVE.load(native_provider::hosts()).unwrap();
    let missing = bindings
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new(
            "missing",
        ))
        .err()
        .unwrap();
    assert_eq!(
        missing.to_string(),
        "function missing does not exist in the Gleam module"
    );
    let mismatch = bindings
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .err()
        .unwrap();
    assert_eq!(
        mismatch.to_string(),
        "function run has type FunctionType { arguments: [], return_: Tuple([Bool, Bool, Int]) }, expected FunctionType { arguments: [], return_: Int }"
    );
    let _run = bindings
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))
        .unwrap();
    let duplicate = bindings
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))
        .err()
        .unwrap();
    assert_eq!(
        duplicate.to_string(),
        "function run was selected more than once"
    );
    assert_eq!(
        duplicate.source().unwrap().downcast_ref::<BindingError>(),
        Some(&BindingError::DuplicateFunction { name: "run".into() })
    );
}

#[test]
fn native_data_matches_preparation_output() {
    use geam_core::embedding::HostedModuleBuilder;
    use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};

    let program = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new(
                "main",
                "src/main.gleam",
                include_str!("fixtures/prepared/native.gleam"),
            )],
        )],
        native_provider::hosts(),
    )
    .unwrap();
    let (bindings, _) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))
        .unwrap();
    assert_eq!(
        bindings.prepare().unwrap().emit_rust(),
        include_str!("fixtures/prepared/native.rs").trim()
    );
    NATIVE.load(native_provider::hosts()).unwrap();
}

#[cfg(feature = "tokio")]
#[test]
fn emitted_native_program_preserves_recursive_values_and_resuming_callbacks() {
    let mut bindings = NATIVE.load(native_provider::hosts()).unwrap();
    let run = bindings
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))
        .unwrap();
    let mut module = bindings.seal();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    let mut echo = Vec::new();
    let value = runtime
        .block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                scope.call(&run, ()).await.unwrap()
            }),
        )
        .unwrap();
    assert_eq!(value, (true, true, BigInt::from(43)));
    assert!(echo.is_empty());
}
