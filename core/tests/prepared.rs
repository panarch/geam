use geam_core::__prepared_support as data;
use geam_core::embedding::{BigInt, BitArrayValue, CallError, FunctionDeclaration, ModuleBuilder};

static ARITHMETIC: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/arithmetic.rs");

static VALUES: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/values.rs");

static NESTED_PATTERNS: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/nested_patterns.rs");

static SPARSE_PATTERNS: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/sparse_patterns.rs");

static BIT_ARRAY_PATTERNS: data::ModuleArtifact<std::convert::Infallible> =
    include!("fixtures/prepared/bit_array_patterns.rs");

#[path = "support/work_fixture.rs"]
mod work_fixture;
#[path = "fixtures/prepared/work_provider.rs"]
mod work_provider;

static WORK: data::HostedModuleArtifact = include!("fixtures/prepared/work.rs");

static ENTRY: data::HostedEntryArtifact = include!("fixtures/prepared/entry.rs");
static ENTRY_WORK: data::HostedEntryArtifact = include!("fixtures/prepared/entry_work.rs");
static ENTRY_FAILURE: data::HostedEntryArtifact = include!("fixtures/prepared/entry_failure.rs");

#[path = "fixtures/prepared/shared_provider.rs"]
mod shared_provider;

static SHARED_CUSTOM: data::HostedModuleArtifact = include!("fixtures/prepared/shared_custom.rs");

#[test]
fn shared_custom_values_preserve_nominal_payloads_and_require_their_producer() {
    assert_eq!(
        shared_provider::prepare().emit_rust(),
        include_str!("fixtures/prepared/shared_custom.rs").trim()
    );
    assert_eq!(
        SHARED_CUSTOM
            .load(shared_provider::hosts(false))
            .err()
            .unwrap()
            .to_string(),
        "prepared provider registration mismatch: Registration { package: \"consumer\", module: \"consumer\", function: \"retain\", reason: SharedCustomType { custom_type: CustomTypeName { package: \"producer\", module: \"handles\", name: \"Handle\" } } }; regenerate with the matching providers"
    );
    SHARED_CUSTOM.load(shared_provider::hosts(true)).unwrap();
    #[cfg(feature = "tokio")]
    {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
        for prepared in [false, true] {
            let (mut module, main) = if prepared {
                let mut bindings = SHARED_CUSTOM.load(shared_provider::hosts(true)).unwrap();
                let main = bindings
                    .function(FunctionDeclaration::<(), (BigInt, BigInt)>::new("main"))
                    .unwrap();
                (bindings.seal(), main)
            } else {
                let typed = geam_core::compile_typed_host_program(
                    "consumer",
                    "consumer",
                    shared_provider::packages(),
                    shared_provider::hosts(true),
                )
                .unwrap();
                let (bindings, main) = geam_core::embedding::HostedModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(), (BigInt, BigInt)>::new("main"))
                    .unwrap();
                (bindings.seal().unwrap(), main)
            };
            for _ in 0..2 {
                let mut echo = Vec::new();
                let result = runtime
                    .block_on(
                        module.with_execution(&host, &mut (), &mut echo, async |scope| {
                            scope.call(&main, ()).await.unwrap()
                        }),
                    )
                    .unwrap();
                assert_eq!(result, (42.into(), 42.into()));
                assert!(echo.is_empty());
            }
        }
    }
}

#[test]
fn nested_constructor_exclusions_preserve_dynamic_and_prepared_results() {
    use geam_core::StringValue;

    let source = include_str!("fixtures/prepared/nested_patterns.gleam");
    let typed = geam_core::compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), StringValue>::new("main"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/nested_patterns.rs").trim()
    );

    for prepared in [false, true] {
        let (module, main) = if prepared {
            let mut bindings = NESTED_PATTERNS.load().unwrap();
            let main = bindings
                .function(FunctionDeclaration::<(), StringValue>::new("main"))
                .unwrap();
            (bindings.seal(), main)
        } else {
            let typed =
                geam_core::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let (bindings, main) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), StringValue>::new("main"))
                .unwrap();
            (bindings.seal(), main)
        };
        for _ in 0..2 {
            let mut echo = Vec::new();
            assert_eq!(
                module.call(&main, (), &mut echo).unwrap().as_str(),
                "present:missing:failed:nested:empty:none:done"
            );
            assert!(echo.is_empty());
        }
    }
}

#[test]
fn unconstructed_pattern_variants_preserve_dynamic_and_prepared_results() {
    use geam_core::StringValue;

    let source = include_str!("fixtures/prepared/sparse_patterns.gleam");
    let typed = geam_core::compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), StringValue>::new("main"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/sparse_patterns.rs").trim()
    );

    for prepared in [false, true] {
        let (module, main) = if prepared {
            let mut bindings = SPARSE_PATTERNS.load().unwrap();
            let main = bindings
                .function(FunctionDeclaration::<(), StringValue>::new("main"))
                .unwrap();
            (bindings.seal(), main)
        } else {
            let typed =
                geam_core::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let (bindings, main) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(), StringValue>::new("main"))
                .unwrap();
            (bindings.seal(), main)
        };
        for _ in 0..2 {
            let mut echo = Vec::new();
            assert_eq!(
                module.call(&main, (), &mut echo).unwrap().as_str(),
                "unnamed:empty:number:fallback"
            );
            assert!(echo.is_empty());
        }
    }
}

#[test]
fn zero_width_bit_array_fields_preserve_dynamic_and_prepared_results() {
    type Fields = (BigInt, f64, BitArrayValue, BigInt);

    let source = include_str!("fixtures/prepared/bit_array_patterns.gleam");
    let typed = geam_core::compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (mut bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BitArrayValue, BigInt), Fields>::new(
            "zero_fields",
        ))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(BitArrayValue, BigInt, BigInt), BigInt>::new("signed_little"),
        )
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/bit_array_patterns.rs").trim()
    );

    for prepared in [false, true] {
        let (module, zero_fields) = if prepared {
            let mut bindings = BIT_ARRAY_PATTERNS.load().unwrap();
            let function = bindings
                .function(FunctionDeclaration::<(BitArrayValue, BigInt), Fields>::new(
                    "zero_fields",
                ))
                .unwrap();
            (bindings.seal(), function)
        } else {
            let typed =
                geam_core::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let (bindings, function) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(BitArrayValue, BigInt), Fields>::new(
                    "zero_fields",
                ))
                .unwrap();
            (bindings.seal(), function)
        };
        for _ in 0..2 {
            for (input, width, expected) in [
                (
                    vec![7],
                    0,
                    (
                        0.into(),
                        0.0,
                        BitArrayValue::from_bytes(Vec::new()),
                        7.into(),
                    ),
                ),
                (
                    vec![7, 8],
                    0,
                    (
                        (-1).into(),
                        -1.0,
                        BitArrayValue::from_bytes(vec![255]),
                        (-1).into(),
                    ),
                ),
                (
                    vec![7],
                    -1,
                    (
                        (-1).into(),
                        -1.0,
                        BitArrayValue::from_bytes(vec![255]),
                        (-1).into(),
                    ),
                ),
            ] {
                let mut echo = Vec::new();
                assert_eq!(
                    module
                        .call(
                            &zero_fields,
                            (BitArrayValue::from_bytes(input), width.into()),
                            &mut echo,
                        )
                        .unwrap(),
                    expected,
                );
                assert!(echo.is_empty());
            }
        }
    }
}

#[test]
fn signed_little_endian_fields_preserve_dynamic_and_prepared_results() {
    for prepared in [false, true] {
        let (module, read) = if prepared {
            let mut bindings = BIT_ARRAY_PATTERNS.load().unwrap();
            let function = bindings
                .function(
                    FunctionDeclaration::<(BitArrayValue, BigInt, BigInt), BigInt>::new(
                        "signed_little",
                    ),
                )
                .unwrap();
            (bindings.seal(), function)
        } else {
            let typed = geam_core::compile_typed_module(
                "example",
                "src/example.gleam",
                include_str!("fixtures/prepared/bit_array_patterns.gleam"),
            )
            .unwrap();
            let (bindings, function) = ModuleBuilder::new(typed)
                .unwrap()
                .function(
                    FunctionDeclaration::<(BitArrayValue, BigInt, BigInt), BigInt>::new(
                        "signed_little",
                    ),
                )
                .unwrap();
            (bindings.seal(), function)
        };
        let input = BitArrayValue::from_bytes(vec![
            0x92, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x13, 0x57, 0x9b, 0xdf, 0x24, 0x68,
            0xac, 0xe0, 0x80,
        ]);
        for _ in 0..2 {
            for (width, offset, expected) in [
                (0, 0, "0"),
                (8, 3, "-111"),
                (12, 3, "-1391"),
                (64, 3, "-9153593911804714351"),
                (65, 3, "-9153593911804714351"),
                (128, 3, "5853120896148130334324824293139260049"),
                (129, 0, "-41640108513433735387645454385746135918"),
                (137, 0, "-1"),
                (-1, 0, "-1"),
                (8, -1, "-1"),
            ] {
                let mut echo = Vec::new();
                assert_eq!(
                    module
                        .call(
                            &read,
                            (input.clone(), width.into(), offset.into()),
                            &mut echo,
                        )
                        .unwrap(),
                    BigInt::parse_bytes(expected.as_bytes(), 10).unwrap(),
                    "prepared {prepared}, width {width}, offset {offset}",
                );
                assert!(echo.is_empty());
            }
        }
    }
}

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
        format: 1,
        ..include!("fixtures/prepared/arithmetic.rs")
    };
    let error = INCOMPATIBLE.load().err().unwrap();
    assert_eq!(
        error.to_string(),
        "prepared format 1 is incompatible with format 4; regenerate the prepared program"
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

#[cfg(feature = "tokio")]
#[test]
fn dynamic_and_prepared_calls_share_captures_through_opaque_values_and_native_work() {
    use work_fixture::WorkType;
    use work_provider::Captured;

    macro_rules! select {
        ($bindings:ident) => {{
            let capture = $bindings
                .function(FunctionDeclaration::<(BigInt,), Captured>::new("capture"))
                .unwrap();
            let extend = $bindings
                .function(FunctionDeclaration::<(Captured, BigInt), Captured>::new(
                    "extend",
                ))
                .unwrap();
            let identities = $bindings
                .function(FunctionDeclaration::<(Captured,), (bool, bool)>::new(
                    "identities",
                ))
                .unwrap();
            let invoke = $bindings
                .function(FunctionDeclaration::<(Captured,), WorkType<BigInt>>::new(
                    "invoke",
                ))
                .unwrap();
            (capture, extend, identities, invoke)
        }};
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, (capture, extend, identities, invoke)) = if prepared {
            let mut bindings = WORK.load(work_provider::hosts()).unwrap();
            let functions = select!(bindings);
            (bindings.seal(), functions)
        } else {
            let (mut bindings, _) =
                geam_core::embedding::HostedModuleBuilder::new(work_provider::program())
                    .unwrap()
                    .function(FunctionDeclaration::<(BigInt,), WorkType<BigInt>>::new(
                        "make",
                    ))
                    .unwrap();
            let functions = select!(bindings);
            (bindings.seal().unwrap(), functions)
        };
        let mut echo = Vec::new();
        // Reuse the same loaded module, but never revive a previous scope's work.
        for _ in 0..3 {
            runtime
                .block_on(
                    module.with_execution(&host, &mut (), &mut echo, async |scope| {
                        let original = scope.call(&capture, (8.into(),)).await.unwrap();
                        let sibling = original.clone();
                        let mut value = original;
                        for _ in 0..8 {
                            value = scope.call(&extend, (&value, 8.into())).await.unwrap();
                        }
                        assert_eq!(scope.call(&identities, (&value,)).await, Ok((true, false)));
                        let work = scope.call(&invoke, (&value,)).await.unwrap();
                        drop(value);
                        let complete = scope.observe(&work).await.unwrap();
                        let alias = scope.observe(&work).await.unwrap();
                        complete
                            .read(|left| alias.read(|right| assert!(std::ptr::eq(left, right))));
                        assert_eq!(complete.read(Clone::clone), BigInt::from(73));
                        let sibling_work = scope.call(&invoke, (&sibling,)).await.unwrap();
                        assert_eq!(
                            scope
                                .observe(&sibling_work)
                                .await
                                .unwrap()
                                .read(Clone::clone),
                            BigInt::from(9)
                        );
                        let deep = scope.call(&capture, (2000.into(),)).await.unwrap();
                        let unobserved = scope.call(&invoke, (&deep,)).await.unwrap();
                        drop(deep);
                        drop(unobserved);
                    }),
                )
                .unwrap();
        }
        assert!(echo.is_empty());
    }
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
    let (mut bindings, _) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (geam_core::StringValue,),
            (bool, geam_core::StringValue),
        >::new("substring"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (geam_core::BitArrayValue, BigInt, BigInt),
            geam_core::BitArrayValue,
        >::new("bit_range"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (geam_core::BitArrayValue,),
            geam_core::BitArrayValue,
        >::new("bit_tail"))
        .unwrap();
    assert_eq!(
        bindings.prepare().unwrap().emit_rust(),
        include_str!("fixtures/prepared/native.rs").trim()
    );
    NATIVE.load(native_provider::hosts()).unwrap();
}

#[cfg(feature = "tokio")]
#[test]
fn dynamic_and_prepared_strings_share_input_storage_after_native_calls() {
    use geam_core::embedding::{HostedModuleBuilder, StringValue};
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
    let (dynamic, dynamic_entry) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(StringValue,), (bool, StringValue)>::new("substring"))
        .unwrap();
    let mut prepared = NATIVE.load(native_provider::hosts()).unwrap();
    let prepared_entry = prepared
        .function(FunctionDeclaration::<(StringValue,), (bool, StringValue)>::new("substring"))
        .unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    for (mut module, entry) in [
        (dynamic.seal().unwrap(), dynamic_entry),
        (prepared.seal(), prepared_entry),
    ] {
        let input = StringValue::from("prefix:abcdefghijklmnopqrstuvwxyz");
        let address = input.as_ptr().addr() + 7;
        let mut echo = Vec::new();
        let (same, text) = runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async move |scope| {
                    scope.call(&entry, (input,)).await.unwrap()
                }),
            )
            .unwrap();
        drop(module);
        assert!(same);
        assert_eq!(text.as_str(), "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(text.as_ptr().addr(), address);
        assert!(echo.is_empty());
    }
}

#[cfg(feature = "tokio")]
#[test]
fn dynamic_and_prepared_bit_ranges_preserve_storage_and_canonical_bytes_through_providers() {
    use geam_core::embedding::{BitArrayValue, HostedModuleBuilder};
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
    let (mut dynamic, dynamic_range) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<
            (BitArrayValue, BigInt, BigInt),
            BitArrayValue,
        >::new("bit_range"))
        .unwrap();
    let dynamic_tail = dynamic
        .function(FunctionDeclaration::<(BitArrayValue,), BitArrayValue>::new(
            "bit_tail",
        ))
        .unwrap();
    let mut prepared = NATIVE.load(native_provider::hosts()).unwrap();
    let prepared_range = prepared
        .function(FunctionDeclaration::<
            (BitArrayValue, BigInt, BigInt),
            BitArrayValue,
        >::new("bit_range"))
        .unwrap();
    let prepared_tail = prepared
        .function(FunctionDeclaration::<(BitArrayValue,), BitArrayValue>::new(
            "bit_tail",
        ))
        .unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());

    for (mut module, range, tail) in [
        (dynamic.seal().unwrap(), dynamic_range, dynamic_tail),
        (prepared.seal(), prepared_range, prepared_tail),
    ] {
        let input = BitArrayValue::from_bytes(vec![0xab, 0xcd, 0xef]);
        let pointer = input.bytes().as_ptr();
        let partial = BitArrayValue::try_from_parts(vec![0xab, 0xcd, 0xef], 20).unwrap();
        let mut echo = Vec::new();
        let (aligned, unaligned, short, partial_source, empty) = runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    let aligned = scope
                        .call(&range, (input.clone(), 8.into(), 8.into()))
                        .await
                        .unwrap();
                    let unaligned = scope
                        .call(&range, (input.clone(), 4.into(), 8.into()))
                        .await
                        .unwrap();
                    let short = scope
                        .call(&range, (input.clone(), 0.into(), 4.into()))
                        .await
                        .unwrap();
                    let partial_source = scope
                        .call(&range, (partial, 8.into(), 8.into()))
                        .await
                        .unwrap();
                    let empty = scope
                        .call(&tail, (BitArrayValue::from_bytes(vec![1]),))
                        .await
                        .unwrap();
                    (aligned, unaligned, short, partial_source, empty)
                }),
            )
            .unwrap();
        drop(input);
        drop(module);
        assert_eq!(aligned.bytes(), &[0xcd]);
        assert_eq!(aligned.bytes().as_ptr(), pointer.wrapping_add(1));
        assert_eq!(unaligned, BitArrayValue::from_bytes(vec![0xbc]));
        assert_eq!(short.bytes(), &[0xa0]);
        assert_eq!(short.bit_len(), 4);
        assert_eq!(partial_source, BitArrayValue::from_bytes(vec![0xcd]));
        assert_eq!(empty, BitArrayValue::from_bytes(Vec::new()));
        assert!(echo.is_empty());
    }
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

#[path = "fixtures/prepared/callable_declarations.rs"]
mod callable_declarations;
#[path = "fixtures/prepared/callable_provider.rs"]
mod callable_provider;

// This ordinary application module is absent from the declaration-only helper.
mod pricing {
    pub(super) fn add(base: num_bigint::BigInt, value: num_bigint::BigInt) -> num_bigint::BigInt {
        base + value
    }
}

static CALLABLES: data::HostedModuleArtifact = include!("fixtures/prepared/callables.rs");

#[cfg(feature = "tokio")]
static EMBEDDED_CALLABLES: data::HostedModuleArtifact =
    include!("fixtures/prepared/callable_embedding.rs");

#[test]
fn scoped_callable_artifact_matches_declaration_only_preparation() {
    assert_eq!(
        callable_declarations::prepare_scoped().emit_rust(),
        include_str!("fixtures/prepared/callable_embedding.rs").trim(),
    );
}

#[cfg(feature = "tokio")]
#[test]
fn scoped_function_inputs_returns_and_nested_codecs_match_in_dynamic_and_prepared_modules() {
    use geam_core::embedding::{CallableType, HostedModuleBuilder, List};
    type Adjust = CallableType<(BigInt,), BigInt>;
    let program = geam_core::compile_typed_host_program(
        "application",
        "library",
        callable_declarations::packages(),
        callable_provider::implementations(),
    )
    .unwrap();
    let (mut dynamic, make) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt,), Adjust>::new("make_native"))
        .unwrap();
    let mut prepared = EMBEDDED_CALLABLES
        .load(callable_provider::implementations())
        .unwrap();
    let prepared_make = prepared
        .function(FunctionDeclaration::<(BigInt,), Adjust>::new("make_native"))
        .unwrap();
    macro_rules! select {
        ($bindings:ident) => {
            (
                $bindings
                    .function(FunctionDeclaration::<(Adjust,), Adjust>::new("keep"))
                    .unwrap(),
                $bindings
                    .function(FunctionDeclaration::<(BigInt, Adjust), BigInt>::new(
                        "calculate",
                    ))
                    .unwrap(),
                $bindings
                    .function(FunctionDeclaration::<(List<Adjust>,), List<Adjust>>::new(
                        "function_list",
                    ))
                    .unwrap(),
                $bindings
                    .function(
                        FunctionDeclaration::<(Adjust,), (Adjust, Result<Adjust, ()>)>::new(
                            "container",
                        ),
                    )
                    .unwrap(),
                $bindings
                    .function(
                        FunctionDeclaration::<(), CallableType<(BigInt,), Adjust>>::new("maker"),
                    )
                    .unwrap(),
                $bindings
                    .function(FunctionDeclaration::<
                        (),
                        CallableType<(List<Result<BigInt, ()>>,), BigInt>,
                    >::new("picker"))
                    .unwrap(),
                $bindings.callable::<callable_declarations::Add>().unwrap(),
                $bindings
                    .callable::<callable_declarations::Constant<bool>>()
                    .unwrap(),
                $bindings
                    .callable::<callable_declarations::Wrap<BigInt, BigInt>>()
                    .unwrap(),
            )
        };
    }
    let dynamic_functions = select!(dynamic);
    let prepared_functions = select!(prepared);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    for (
        mut module,
        make,
        (keep, calculate, list, container, maker, picker, native, constant, wrap),
    ) in [
        (dynamic.seal().unwrap(), make, dynamic_functions),
        (prepared.seal(), prepared_make, prepared_functions),
    ] {
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut drop, async |scope| {
                    let source = scope.call(&make, (BigInt::from(20),)).await.unwrap();
                    assert_eq!(
                        scope.invoke(&source, (BigInt::from(2),)).await.unwrap(),
                        BigInt::from(22)
                    );
                    let constant = scope.construct(&constant, (true, ())).unwrap();
                    assert!(scope.invoke(&constant, ()).await.unwrap());
                    let native = scope.construct(&native, (BigInt::from(10), ())).unwrap();
                    let add = scope.construct(&wrap, (&native, ())).unwrap();
                    assert_eq!(
                        scope
                            .call(&calculate, (BigInt::from(5), &add))
                            .await
                            .unwrap(),
                        BigInt::from(15)
                    );
                    let alias = scope.call(&keep, (&add,)).await.unwrap();
                    assert_eq!(
                        scope.invoke(&alias, (BigInt::from(7),)).await.unwrap(),
                        BigInt::from(17)
                    );
                    let functions = scope.call(&list, (vec![&add, &alias],)).await.unwrap();
                    let retained = functions.read_item(1, std::convert::identity).unwrap();
                    assert_eq!(
                        scope.invoke(&retained, (BigInt::from(32),)).await.unwrap(),
                        BigInt::from(42)
                    );
                    let (first, second) = scope.call(&container, (&add,)).await.unwrap();
                    assert_eq!(
                        scope.invoke(&first, (BigInt::from(1),)).await.unwrap(),
                        BigInt::from(11)
                    );
                    assert_eq!(
                        scope
                            .invoke(&second.unwrap(), (BigInt::from(2),))
                            .await
                            .unwrap(),
                        BigInt::from(12)
                    );
                    let factory = scope.call(&maker, ()).await.unwrap();
                    let add = scope.invoke(&factory, (BigInt::from(40),)).await.unwrap();
                    assert_eq!(
                        scope.invoke(&add, (BigInt::from(2),)).await.unwrap(),
                        BigInt::from(42)
                    );
                    let pick = scope.call(&picker, ()).await.unwrap();
                    assert_eq!(
                        scope
                            .invoke(&pick, (vec![Ok(BigInt::from(42))],))
                            .await
                            .unwrap(),
                        BigInt::from(42)
                    );
                }),
            )
            .unwrap();
    }
}

#[test]
fn native_callable_artifact_uses_declarations_only_and_requires_fresh_body_bindings() {
    assert_eq!(
        callable_declarations::prepare().emit_rust(),
        include_str!("fixtures/prepared/callables.rs").trim()
    );
    let program = geam_core::compile_typed_host_program(
        "application",
        "library",
        callable_declarations::packages(),
        callable_provider::implementations(),
    )
    .unwrap();
    let (mut actual_bodies, _) = geam_core::embedding::HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .unwrap();
    actual_bodies
        .function(FunctionDeclaration::<(), bool>::new("check"))
        .unwrap();
    assert_eq!(
        actual_bodies.prepare().unwrap().emit_rust(),
        callable_declarations::prepare().emit_rust()
    );
    CALLABLES
        .load(callable_provider::implementations())
        .unwrap();
    let missing = geam_core::HostProviderSet::<geam_core::StatelessHostProfile>::new([]).unwrap();
    assert!(CALLABLES.load(missing).is_err());
}

#[cfg(feature = "tokio")]
#[test]
fn declaration_only_callable_artifacts_run_app_bodies_with_dynamic_capture_and_identity_parity() {
    use geam_core::embedding::HostedModuleBuilder;
    let program = geam_core::compile_typed_host_program(
        "application",
        "library",
        callable_declarations::packages(),
        callable_provider::implementations(),
    )
    .unwrap();
    let (mut dynamic, run) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .unwrap();
    let check = dynamic
        .function(FunctionDeclaration::<(), bool>::new("check"))
        .unwrap();
    let mut prepared = CALLABLES
        .load(callable_provider::implementations())
        .unwrap();
    let prepared_run = prepared
        .function(FunctionDeclaration::<(), BigInt>::new("run"))
        .unwrap();
    let prepared_check = prepared
        .function(FunctionDeclaration::<(), bool>::new("check"))
        .unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    for (mut module, run, check) in [
        (dynamic.seal().unwrap(), run, check),
        (prepared.seal(), prepared_run, prepared_check),
    ] {
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut drop, async |scope| {
                    assert_eq!(scope.call(&run, ()).await.unwrap(), BigInt::from(42));
                    assert!(scope.call(&check, ()).await.unwrap());
                }),
            )
            .unwrap();
    }
}

static NATIVE_VIEWS: data::HostedModuleArtifact = include!("fixtures/prepared/callable_views.rs");

#[test]
fn native_view_artifact_matches_declaration_only_preparation() {
    assert_eq!(
        callable_declarations::prepare_native_views().emit_rust(),
        include_str!("fixtures/prepared/callable_views.rs").trim()
    );
}

#[cfg(feature = "tokio")]
#[test]
fn native_construction_selects_exact_rust_views_in_dynamic_and_prepared_execution() {
    use geam_core::embedding::{CallableType, HostedModuleBuilder};
    use geam_core::provider::ProviderResult;
    type Outcome = Result<BigInt, ()>;
    type Callback = CallableType<(BigInt,), Outcome>;
    type Constant = callable_declarations::Constant<ProviderResult<BigInt, ()>>;
    type Wrap = callable_declarations::Wrap<BigInt, ProviderResult<BigInt, ()>>;
    let program = geam_core::compile_typed_host_program(
        "application",
        "library",
        callable_declarations::packages(),
        callable_provider::implementations(),
    )
    .unwrap();
    let (mut dynamic, source) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(), Callback>::new("result_function"))
        .unwrap();
    dynamic.callable::<Constant>().unwrap();
    let dynamic_constant = dynamic
        .callable_as::<Constant, (), Outcome, (Outcome, ())>()
        .unwrap();
    let dynamic_wrap = dynamic
        .callable_as::<Wrap, (BigInt,), Outcome, (Callback, ())>()
        .unwrap();
    let mut prepared = NATIVE_VIEWS
        .load(callable_provider::implementations())
        .unwrap();
    let prepared_source = prepared
        .function(FunctionDeclaration::<(), Callback>::new("result_function"))
        .unwrap();
    let prepared_constant = prepared
        .callable_as::<Constant, (), Outcome, (Outcome, ())>()
        .unwrap();
    let prepared_wrap = prepared
        .callable_as::<Wrap, (BigInt,), Outcome, (Callback, ())>()
        .unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
    for (mut module, source, constant, wrap) in [
        (
            dynamic.seal().unwrap(),
            source,
            dynamic_constant,
            dynamic_wrap,
        ),
        (
            prepared.seal(),
            prepared_source,
            prepared_constant,
            prepared_wrap,
        ),
    ] {
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut drop, async |scope| {
                    let constant = scope
                        .construct(&constant, (Ok(BigInt::from(42)), ()))
                        .unwrap();
                    assert_eq!(
                        scope.invoke(&constant, ()).await.unwrap(),
                        Ok(BigInt::from(42))
                    );
                    let callback = scope.call(&source, ()).await.unwrap();
                    let wrapped = scope.construct(&wrap, (&callback, ())).unwrap();
                    assert_eq!(
                        scope.invoke(&wrapped, (BigInt::from(7),)).await.unwrap(),
                        Ok(BigInt::from(7))
                    );
                }),
            )
            .unwrap();
    }
}

#[test]
fn malformed_native_construction_metadata_is_rejected_before_execution() {
    enum Change {
        BodyIdentity,
        Completion,
        Parameters,
        Captures,
        Invocation,
    }
    for change in [
        Change::BodyIdentity,
        Change::Completion,
        Change::Parameters,
        Change::Captures,
        Change::Invocation,
    ] {
        const ARTIFACT: data::HostedModuleArtifact =
            include!("fixtures/prepared/callable_embedding.rs");
        let mut artifact = ARTIFACT;
        let mut entries = artifact.callables.to_vec();
        let entry = &mut entries[0];
        assert_eq!(entry.declaration.name.as_ref(), "add");
        match change {
            Change::BodyIdentity => entry.declaration.name = "another_body".into(),
            Change::Completion => entry.declaration.returns_value = false,
            Change::Parameters => entry.construction.parameters = data::Storage::Static(&[]),
            Change::Captures => entry.construction.captures = data::Storage::Static(&[]),
            Change::Invocation => {
                entry.invocation.type_.return_ =
                    data::Storage::Owned(Box::new(data::type_::ValueType::Bool))
            }
        }
        artifact.callables = entries.into();
        let artifact = Box::leak(Box::new(artifact));
        let error = artifact
            .load(callable_provider::implementations())
            .err()
            .unwrap();
        assert_eq!(
            error.to_string(),
            "prepared provider registration mismatch: Call(Callable); regenerate with the matching providers"
        );
    }
}

#[test]
fn native_selection_requires_a_prepared_exact_declaration_and_rust_view() {
    use geam_core::provider::ProviderResult;
    type Outcome = Result<BigInt, ()>;
    type Constant = callable_declarations::Constant<ProviderResult<BigInt, ()>>;
    let mut prepared = NATIVE_VIEWS
        .load(callable_provider::implementations())
        .unwrap();
    let error = prepared
        .callable::<callable_declarations::Add>()
        .err()
        .unwrap();
    assert_eq!(
        error.to_string(),
        "native callable support:support/private.add is missing or has an incompatible declaration"
    );
    let error = prepared
        .callable_as::<Constant, (), bool, (Outcome, ())>()
        .err()
        .unwrap();
    assert_eq!(
        error.to_string(),
        "Rust views do not match the exact types of native callable support:support/private.constant"
    );

    const ARTIFACT: data::HostedModuleArtifact = include!("fixtures/prepared/callable_views.rs");
    let mut artifact = ARTIFACT;
    let mut entries = artifact.callables.to_vec();
    entries.remove(1); // Only the opaque Result capture codec remains for Constant.
    artifact.callables = entries.into();
    let artifact = Box::leak(Box::new(artifact));
    let mut prepared = artifact.load(callable_provider::implementations()).unwrap();
    let error = prepared
        .callable_as::<Constant, (), Outcome, (Outcome, ())>()
        .err()
        .unwrap();
    assert_eq!(
        error.to_string(),
        "function constant has incompatible prepared Rust inputs: VariantCount { expected: 1, actual: 0 }; regenerate with the matching declarations"
    );
}
