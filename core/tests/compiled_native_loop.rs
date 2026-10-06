use geam_core::__prepared_support as data;
use geam_core::compile_typed_host_program;
use geam_core::embedding::{
    BigInt, BitArrayValue, FunctionDeclaration, HostedModuleBuilder, List, StringValue,
};

#[path = "fixtures/prepared/native_loop_provider.rs"]
mod native_loop_provider;

static NATIVE_LOOP: data::HostedModuleArtifact = include!("fixtures/prepared/native_loop.rs");

#[test]
fn public_generation_keeps_the_native_loop_and_existing_call_fallback() {
    assert_eq!(
        native_loop_provider::prepare().emit_rust(),
        include_str!("fixtures/prepared/native_loop.rs").trim()
    );
    let loops = &NATIVE_LOOP.module.program.compiled.native_loops;
    let expected_families = [3, 2, 2, 2, 1, 2, 1];
    let mut families = [0; 7];
    for row in loops.iter() {
        let data::compiled::CompiledImplementation::NativeLoop(implementation) =
            &row.implementation
        else {
            panic!("native-loop table must contain native loops");
        };
        assert_eq!(row.function, implementation.function);
        let family = match row.function {
            data::compiled::NativeLoopTarget::Int(_) => 0,
            data::compiled::NativeLoopTarget::Float(_) => 1,
            data::compiled::NativeLoopTarget::String(_) => 2,
            data::compiled::NativeLoopTarget::BitArray(_) => 3,
            data::compiled::NativeLoopTarget::UtfCodepoint(_) => 4,
            data::compiled::NativeLoopTarget::Bool(_) => 5,
            data::compiled::NativeLoopTarget::Nil(_) => 6,
        };
        families[family] += 1;
    }
    assert_eq!(families, expected_families);
    NATIVE_LOOP
        .load(native_loop_provider::hosts(false))
        .unwrap();
    NATIVE_LOOP.load(native_loop_provider::hosts(true)).unwrap();
}

#[cfg(feature = "tokio")]
#[test]
fn graph_callees_and_artifacts_without_native_loops_keep_ordinary_execution() {
    use geam_core::HostProviderSet;
    use geam_core::StatelessHostProfile;
    use geam_core::execution::TokioHost;

    static NUMERIC: data::HostedModuleArtifact = include!("fixtures/prepared/numeric_hosted.rs");
    static STOP: data::HostedModuleArtifact =
        include!("fixtures/prepared/string_checkpoint_hosted_stop.rs");
    assert!(NUMERIC.module.program.compiled.native_loops.is_empty());
    assert!(STOP.module.program.compiled.native_loops.is_empty());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut bindings = NUMERIC
        .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
        .unwrap();
    let arithmetic = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "arithmetic",
        ))
        .unwrap();
    let mut module = bindings.seal();
    assert_eq!(
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                    scope
                        .call(&arithmetic, (BigInt::from(4), BigInt::from(0)))
                        .await
                })
            )
            .unwrap()
            .try_into_value()
            .unwrap(),
        Ok(BigInt::from(26))
    );

    let mut bindings = STOP
        .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
        .unwrap();
    let stop = bindings
        .function(FunctionDeclaration::<(StringValue,), List<BigInt>>::new(
            "list_stop",
        ))
        .unwrap();
    let mut module = bindings.seal();
    let error = runtime
        .block_on(
            module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                scope
                    .call(&stop, (StringValue::from("ordinary graph"),))
                    .await
                    .map(|value| value.len())
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap()
        .unwrap_err();
    assert_eq!(error.to_string(), "panic: ordinary graph");

    let mut bindings = NATIVE_LOOP.load(native_loop_provider::hosts(true)).unwrap();
    let graph = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "graph_captured",
        ))
        .unwrap();
    let mut module = bindings.seal();
    assert_eq!(
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                    scope
                        .call(&graph, (BigInt::from(129), BigInt::from(7)))
                        .await
                })
            )
            .unwrap()
            .try_into_value()
            .unwrap(),
        Ok(BigInt::from(7))
    );
}

#[cfg(feature = "tokio")]
#[test]
fn generated_and_canonical_loops_preserve_each_native_result_failure_and_cancellation() {
    use geam_core::ExecutionError;
    use geam_core::embedding::CallError;
    use geam_core::execution::TokioHost;
    use native_loop_provider::{AUDIT, Audit};
    let _serial = native_loop_provider::TEST_LOCK.lock().unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut canonical_failure = None;
    for prepared in [false, true] {
        for retained in [false, true] {
            let (mut module, captured, computed, cancellable, opaque, compound) = if prepared {
                let mut bindings = NATIVE_LOOP
                    .load(native_loop_provider::hosts(retained))
                    .unwrap();
                let captured = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "captured",
                    ))
                    .unwrap();
                let computed = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "computed",
                    ))
                    .unwrap();
                let cancellable = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "cancellable",
                    ))
                    .unwrap();
                let opaque = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "retained_value",
                    ))
                    .unwrap();
                let compound = bindings
                    .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
                        "compound",
                    ))
                    .unwrap();
                (
                    bindings.seal(),
                    captured,
                    computed,
                    cancellable,
                    opaque,
                    compound,
                )
            } else {
                let typed = compile_typed_host_program(
                    "application",
                    "native_loop",
                    native_loop_provider::packages(),
                    native_loop_provider::hosts(retained),
                )
                .unwrap();
                let (mut bindings, captured) = HostedModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "captured",
                    ))
                    .unwrap();
                let computed = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "computed",
                    ))
                    .unwrap();
                let cancellable = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "cancellable",
                    ))
                    .unwrap();
                let opaque = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "retained_value",
                    ))
                    .unwrap();
                let compound = bindings
                    .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
                        "compound",
                    ))
                    .unwrap();
                (
                    bindings.seal().unwrap(),
                    captured,
                    computed,
                    cancellable,
                    opaque,
                    compound,
                )
            };
            for count in [1, 2, 5, 127, 128, 129, 10_000] {
                for (function, input, expected_retained) in [
                    (&captured, 7, if prepared && retained { count } else { 0 }),
                    (&computed, 8, 0),
                ] {
                    *AUDIT.lock().unwrap() = Audit::default();
                    let result = runtime
                        .block_on(module.with_execution(
                            &host,
                            &mut (),
                            &mut Vec::new(),
                            async |scope| {
                                scope
                                    .call(function, (BigInt::from(count), BigInt::from(7)))
                                    .await
                            },
                        ))
                        .unwrap()
                        .try_into_value()
                        .unwrap();
                    assert_eq!(result, Ok(BigInt::from(input + count)));
                    let audit = AUDIT.lock().unwrap();
                    assert_eq!(audit.inputs, vec![BigInt::from(input); count as usize]);
                    assert_eq!(audit.retained, expected_retained as usize);
                }
            }
            assert_eq!(
                runtime
                    .block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut Vec::new(),
                        async |scope| {
                            scope
                                .call(&opaque, (BigInt::from(129), BigInt::from(i128::MAX) + 1))
                                .await
                        }
                    ))
                    .unwrap()
                    .try_into_value()
                    .unwrap(),
                Ok(BigInt::from(i128::MAX) + 1)
            );
            let values = vec![BigInt::from(3), BigInt::from(7)];
            assert_eq!(
                runtime
                    .block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut Vec::new(),
                        async |scope| {
                            let values = scope.call(&compound, (values,)).await.unwrap();
                            (
                                values.len(),
                                values.read_item(0, Clone::clone),
                                values.read_item(1, Clone::clone),
                                values.read_item(2, Clone::clone),
                            )
                        }
                    ))
                    .unwrap()
                    .try_into_value()
                    .unwrap(),
                (2, Some(BigInt::from(3)), Some(BigInt::from(7)), None)
            );

            for stop_at in [1, 3, 5] {
                *AUDIT.lock().unwrap() = Audit {
                    fail_at: Some(stop_at),
                    ..Audit::default()
                };
                let failure = runtime
                    .block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut Vec::new(),
                        async |scope| {
                            scope
                                .call(&captured, (BigInt::from(5), BigInt::from(7)))
                                .await
                        },
                    ))
                    .unwrap()
                    .try_into_value()
                    .unwrap()
                    .unwrap_err();
                if canonical_failure.is_none() {
                    canonical_failure = Some(failure.clone());
                }
                assert_eq!(failure, canonical_failure.clone().unwrap());
                let CallError::Execution(ExecutionError::Host(error)) = &failure else {
                    panic!("native failure must retain its host diagnostic");
                };
                assert_eq!(error.failure().to_string(), "observed native failure");
                assert_eq!(AUDIT.lock().unwrap().inputs, vec![7.into(); stop_at]);

                *AUDIT.lock().unwrap() = Audit {
                    cancel_at: Some(stop_at),
                    ..Audit::default()
                };
                let cancelled = runtime
                    .block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut Vec::new(),
                        async |scope| {
                            scope
                                .call(&cancellable, (BigInt::from(5), BigInt::from(7)))
                                .await
                        },
                    ))
                    .unwrap();
                assert_eq!(
                    cancelled.try_into_value().unwrap(),
                    Err(CallError::Cancelled)
                );
                assert_eq!(AUDIT.lock().unwrap().inputs, vec![7.into(); stop_at]);
            }
            for count in [
                BigInt::from(0),
                BigInt::from(-3),
                BigInt::from(i64::MAX) + 1,
            ] {
                *AUDIT.lock().unwrap() = Audit {
                    cancel_at: Some(5),
                    ..Audit::default()
                };
                let cancelled = runtime
                    .block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut Vec::new(),
                        async |scope| scope.call(&cancellable, (count, BigInt::from(7))).await,
                    ))
                    .unwrap();
                assert_eq!(
                    cancelled.try_into_value().unwrap(),
                    Err(CallError::Cancelled)
                );
                assert_eq!(AUDIT.lock().unwrap().inputs, vec![7.into(); 5]);
            }
            for stop_at in [1, 3, 5] {
                *AUDIT.lock().unwrap() = Audit {
                    panic_at: Some(stop_at),
                    ..Audit::default()
                };
                let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    runtime.block_on(module.with_execution(
                        &host,
                        &mut (),
                        &mut Vec::new(),
                        async |scope| {
                            scope
                                .call(&captured, (BigInt::from(5), BigInt::from(7)))
                                .await
                        },
                    ))
                }))
                .unwrap_err();
                assert_eq!(panic.downcast_ref::<&str>(), Some(&"observed Rust panic"));
                assert_eq!(AUDIT.lock().unwrap().inputs, vec![7.into(); stop_at]);
            }
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn primitive_loops_select_the_shared_engine_and_preserve_each_input_and_actual_return() {
    use geam_core::execution::TokioHost;
    use native_loop_provider::{
        KEEP_CALLS, KEEP_RETAINED, PRIMITIVES, PrimitiveAudit, PrimitiveInput,
    };
    use std::sync::atomic::Ordering;
    let _serial = native_loop_provider::TEST_LOCK.lock().unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    macro_rules! bind_primitives {
        ($bindings:ident, $float:ident, $seal:expr) => {{
            let string = $bindings
                .function(
                    FunctionDeclaration::<(BigInt, StringValue), StringValue>::new(
                        "captured_string",
                    ),
                )
                .unwrap();
            let bits = $bindings
                .function(
                    FunctionDeclaration::<(BigInt, BitArrayValue), BitArrayValue>::new(
                        "captured_bit_array",
                    ),
                )
                .unwrap();
            let codepoint = $bindings
                .function(FunctionDeclaration::<(BigInt, char), char>::new(
                    "captured_utf_codepoint",
                ))
                .unwrap();
            let boolean = $bindings
                .function(FunctionDeclaration::<(BigInt, bool), bool>::new(
                    "captured_bool",
                ))
                .unwrap();
            let nil = $bindings
                .function(FunctionDeclaration::<(BigInt, ()), ()>::new("captured_nil"))
                .unwrap();
            let mixed = $bindings
                .function(FunctionDeclaration::<(BigInt, f64), bool>::new("mixed"))
                .unwrap();
            let literal = $bindings
                .function(FunctionDeclaration::<(BigInt,), ()>::new("literal_nil"))
                .unwrap();
            let computed = $bindings
                .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
                    "computed_float",
                ))
                .unwrap();
            let keep_float = $bindings
                .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
                    "retained_float",
                ))
                .unwrap();
            let keep_string = $bindings
                .function(
                    FunctionDeclaration::<(BigInt, StringValue), StringValue>::new(
                        "retained_string",
                    ),
                )
                .unwrap();
            let keep_bits = $bindings
                .function(
                    FunctionDeclaration::<(BigInt, BitArrayValue), BitArrayValue>::new(
                        "retained_bit_array",
                    ),
                )
                .unwrap();
            (
                $seal,
                $float,
                string,
                bits,
                codepoint,
                boolean,
                nil,
                mixed,
                literal,
                computed,
                keep_float,
                keep_string,
                keep_bits,
            )
        }};
    }
    for prepared in [false, true] {
        for retained in [false, true] {
            let (
                mut module,
                float,
                string,
                bits,
                codepoint,
                boolean,
                nil,
                mixed,
                literal,
                computed,
                keep_float,
                keep_string,
                keep_bits,
            ) = if prepared {
                let mut bindings = NATIVE_LOOP
                    .load(native_loop_provider::hosts(retained))
                    .unwrap();
                let float = bindings
                    .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
                        "captured_float",
                    ))
                    .unwrap();
                bind_primitives!(bindings, float, bindings.seal())
            } else {
                let typed = compile_typed_host_program(
                    "application",
                    "native_loop",
                    native_loop_provider::packages(),
                    native_loop_provider::hosts(retained),
                )
                .unwrap();
                let (mut bindings, float) = HostedModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(BigInt, f64), f64>::new(
                        "captured_float",
                    ))
                    .unwrap();
                bind_primitives!(bindings, float, bindings.seal().unwrap())
            };
            macro_rules! call {
                ($function:expr, $arguments:expr) => {
                    runtime
                        .block_on(module.with_execution(
                            &host,
                            &mut (),
                            &mut Vec::new(),
                            async |scope| scope.call($function, $arguments).await,
                        ))
                        .unwrap()
                        .try_into_value()
                        .unwrap()
                        .unwrap()
                };
            }
            let original_string: StringValue = "shared unicode λ".repeat(2048).into();
            let raw_string = StringValue::from_bytes(vec![0xff; 4096]).slice(1..4095);
            let original_bits = BitArrayValue::try_from_parts(vec![0xb7, 0xc8], 13).unwrap();
            for count in [1usize, 2, 5, 127, 128, 129, 10_000] {
                let fast = prepared && retained;
                macro_rules! audit_call {
                    ($function:expr, $arguments:expr, $expected:expr, $input:expr, $fast:expr) => {{
                        *PRIMITIVES.lock().unwrap() = PrimitiveAudit::default();
                        let result = call!($function, $arguments);
                        assert_eq!(result, $expected);
                        let audit = PRIMITIVES.lock().unwrap();
                        assert_eq!(audit.inputs, vec![$input; count]);
                        assert_eq!(audit.retained, if $fast { count } else { 0 });
                    }};
                }
                audit_call!(
                    &float,
                    (count.into(), -0.0),
                    count as f64,
                    PrimitiveInput::Float((-0.0_f64).to_bits()),
                    fast
                );
                audit_call!(
                    &string,
                    (count.into(), original_string.clone()),
                    StringValue::from(if count % 2 == 1 {
                        "native odd"
                    } else {
                        "native even"
                    }),
                    PrimitiveInput::String(original_string.clone()),
                    fast
                );
                audit_call!(
                    &bits,
                    (count.into(), original_bits.clone()),
                    BitArrayValue::from_bytes(vec![(count % 256) as u8]),
                    PrimitiveInput::BitArray(original_bits.clone()),
                    fast
                );
                audit_call!(
                    &codepoint,
                    (count.into(), '🦀'),
                    if count % 2 == 1 { 'β' } else { 'λ' },
                    PrimitiveInput::UtfCodepoint('🦀'),
                    fast
                );
                audit_call!(
                    &boolean,
                    (count.into(), true),
                    count.is_multiple_of(2),
                    PrimitiveInput::Bool(true),
                    fast
                );
                audit_call!(&nil, (count.into(), ()), (), PrimitiveInput::Nil, fast);
                audit_call!(
                    &mixed,
                    (count.into(), 1.25),
                    count % 2 == 1,
                    PrimitiveInput::Float(1.25_f64.to_bits()),
                    fast
                );
                audit_call!(&literal, (count.into(),), (), PrimitiveInput::Nil, fast);
                audit_call!(
                    &computed,
                    (count.into(), 1.25),
                    2.25 + count as f64,
                    PrimitiveInput::Float(2.25_f64.to_bits()),
                    false
                );
            }
            for input in [-0.0, f64::INFINITY, f64::from_bits(0x7ff8_0000_0000_0042)] {
                KEEP_CALLS.store(0, Ordering::Relaxed);
                KEEP_RETAINED.store(0, Ordering::Relaxed);
                let result = call!(&keep_float, (129.into(), input));
                assert_eq!(result.to_bits(), input.to_bits());
                assert_eq!(KEEP_CALLS.load(Ordering::Relaxed), 129);
                assert_eq!(
                    KEEP_RETAINED.load(Ordering::Relaxed),
                    if prepared && retained { 129 } else { 0 }
                );
            }
            for input in [&original_string, &raw_string] {
                KEEP_CALLS.store(0, Ordering::Relaxed);
                KEEP_RETAINED.store(0, Ordering::Relaxed);
                let result = call!(&keep_string, (129.into(), input.clone()));
                assert_eq!(result.as_bytes(), input.as_bytes());
                assert_eq!(result.as_ptr(), input.as_ptr());
                assert_eq!(KEEP_CALLS.load(Ordering::Relaxed), 129);
                assert_eq!(
                    KEEP_RETAINED.load(Ordering::Relaxed),
                    if prepared && retained { 129 } else { 0 }
                );
            }
            *PRIMITIVES.lock().unwrap() = PrimitiveAudit::default();
            assert_eq!(
                call!(&string, (129.into(), raw_string.clone())),
                StringValue::from("native odd")
            );
            {
                let audit = PRIMITIVES.lock().unwrap();
                assert_eq!(
                    audit.inputs,
                    vec![PrimitiveInput::String(raw_string.clone()); 129]
                );
                assert_eq!(audit.retained, if prepared && retained { 129 } else { 0 });
            }
            *PRIMITIVES.lock().unwrap() = PrimitiveAudit::default();
            let raw_result = call!(&keep_string, (129.into(), raw_string.clone()));
            KEEP_CALLS.store(0, Ordering::Relaxed);
            KEEP_RETAINED.store(0, Ordering::Relaxed);
            let result = call!(&keep_bits, (129.into(), original_bits.clone()));
            assert_eq!(result, original_bits);
            assert_eq!(result.bytes().as_ptr(), original_bits.bytes().as_ptr());
            assert_eq!(KEEP_CALLS.load(Ordering::Relaxed), 129);
            assert_eq!(
                KEEP_RETAINED.load(Ordering::Relaxed),
                if prepared && retained { 129 } else { 0 }
            );
            drop(module);
            drop(raw_string);
            assert_eq!(raw_result.as_bytes(), vec![0xff; 4094]);
            assert_eq!(result.bit_len(), 13);
            assert_eq!(result.bytes(), &[0xb7, 0xc8]);
        }
    }
}
