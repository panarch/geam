use geam_core::__prepared_support as data;
use geam_core::embedding::{BigInt, CallError, FunctionDeclaration, ModuleBuilder};
use geam_core::{ExecutionError, PanicKind, PanicMessage, compile_typed_module};
use std::convert::Infallible;
use std::sync::Mutex;

#[cfg(feature = "tokio")]
#[path = "support/work_representation.rs"]
mod work_representation;

static LOOP: data::ModuleArtifact<Infallible> = include!("fixtures/prepared/custom_loop.rs");
const SOURCE: &str = include_str!("fixtures/prepared/custom_loop.gleam");

macro_rules! functions {
    ($bindings:ident, $run:expr) => {
        (
            $run,
            $bindings
                .function(FunctionDeclaration::<(BigInt, bool), BigInt>::new("chosen"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                    "captured",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(bool, BigInt), bool>::new("boolean"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("assertion"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(), BigInt>::new("empty"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("panic_case"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("unsupported"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "custom_capture",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, BigInt), BigInt>::new("repeated"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new(
                    "caller_overflow",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                    "guarded",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(), BigInt>::new("main"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("markers"))
                .unwrap(),
        )
    };
}

#[test]
fn generated_connection_preserves_actual_callbacks_captures_big_values_and_errors() {
    let typed = compile_typed_module("example", "src/custom_loop.gleam", SOURCE).unwrap();
    let (mut bindings, run) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new("run"))
        .unwrap();
    let _ = functions!(bindings, run);
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/custom_loop.rs").trim()
    );
    assert!(LOOP.program.compiled.ints.iter().any(|target| matches!(
        target.implementation,
        data::compiled::CompiledImplementation::CustomLoop(_)
    )));
    assert!(LOOP.program.compiled.bools.iter().any(|target| matches!(
        target.implementation,
        data::compiled::CompiledImplementation::CustomLoop(_)
    )));
    assert!(!LOOP.program.compiled.callbacks.ints.is_empty());
    assert!(!LOOP.program.compiled.callbacks.bools.is_empty());
    let big: BigInt = BigInt::from(1) << 180;
    let mut canonical_panics = Vec::new();
    for prepared in [false, true] {
        let (module, handles) = if prepared {
            let mut bindings = LOOP.load().unwrap();
            let run = bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new("run"))
                .unwrap();
            let handles = functions!(bindings, run);
            (bindings.seal(), handles)
        } else {
            let typed = compile_typed_module("example", "src/custom_loop.gleam", SOURCE).unwrap();
            let (mut bindings, run) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new("run"))
                .unwrap();
            let handles = functions!(bindings, run);
            (bindings.seal(), handles)
        };
        let (
            run,
            chosen,
            captured,
            boolean,
            assertion,
            empty,
            panic_case,
            unsupported,
            custom_capture,
            repeated,
            caller_overflow,
            guarded,
            main,
            markers,
        ) = handles;
        for seed in [BigInt::from(4), big.clone()] {
            assert_eq!(
                module
                    .call(&markers, (seed.clone(),), &mut Vec::new())
                    .unwrap(),
                seed + 6
            );
        }
        for (seed, value) in [
            (3.into(), 9.into()),
            (3.into(), big.clone()),
            (big.clone(), 9.into()),
            (i64::MAX.into(), 1.into()),
        ] {
            assert_eq!(
                module
                    .call(&run, (seed.clone(), value), &mut Vec::new())
                    .unwrap(),
                seed + 7
            );
        }
        for doubled in [false, true] {
            assert_eq!(
                module
                    .call(&chosen, (3.into(), doubled), &mut Vec::new())
                    .unwrap(),
                BigInt::from(if doubled { 13 } else { 8 })
            );
        }
        for enabled in [false, true] {
            for bias in [BigInt::from(4), big.clone()] {
                assert_eq!(
                    module
                        .call(
                            &captured,
                            (3.into(), bias.clone(), enabled),
                            &mut Vec::new()
                        )
                        .unwrap(),
                    if enabled {
                        BigInt::from(8) + bias
                    } else {
                        BigInt::from(3)
                    }
                );
            }
        }
        for initial in [false, true] {
            for value in [
                BigInt::from(-3),
                0.into(),
                7.into(),
                big.clone(),
                -big.clone(),
            ] {
                assert_eq!(
                    module
                        .call(&boolean, (initial, value.clone()), &mut Vec::new())
                        .unwrap(),
                    initial || value > BigInt::from(0)
                );
            }
        }
        for enabled in [false, true] {
            for value in [(-2).into(), 0.into(), 4.into(), big.clone()] {
                let expected = BigInt::from(9)
                    + if enabled && value > BigInt::from(0) {
                        value.clone() * 2
                    } else {
                        BigInt::from(0)
                    };
                assert_eq!(
                    module
                        .call(&guarded, (3.into(), value, enabled), &mut Vec::new())
                        .unwrap(),
                    expected
                );
            }
        }
        assert_eq!(
            module.call(&empty, (), &mut Vec::new()).unwrap(),
            BigInt::from(19)
        );
        assert_eq!(
            module
                .call(&unsupported, (big.clone(),), &mut Vec::new())
                .unwrap(),
            &big + 1
        );
        for bias in [BigInt::from(4), big.clone()] {
            assert_eq!(
                module
                    .call(&custom_capture, (3.into(), bias.clone()), &mut Vec::new())
                    .unwrap(),
                BigInt::from(8) + bias
            );
        }
        for count in [0, 1, 31, 2000] {
            assert_eq!(
                module
                    .call(
                        &repeated,
                        (count.into(), 3.into(), 7.into()),
                        &mut Vec::new()
                    )
                    .unwrap(),
                BigInt::from(7 + 3 * count)
            );
        }
        for seed in [BigInt::from(i64::MAX), big.clone(), BigInt::from(7)] {
            assert_eq!(
                module
                    .call(&caller_overflow, (seed.clone(),), &mut Vec::new())
                    .unwrap(),
                seed + 3
            );
        }
        assert_eq!(
            module.call(&main, (), &mut Vec::new()).unwrap(),
            BigInt::from(50)
        );
        let mut panics = Vec::new();
        for value in [BigInt::from(7), big.clone()] {
            let error = module
                .call(&assertion, (value.clone(),), &mut Vec::new())
                .unwrap_err()
                .into_materialized();
            let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                panic!("assertion error");
            };
            assert_eq!(panic.kind(), PanicKind::LetAssert);
            assert_eq!(panic.site().function(), "asserted");
            panics.push(panic);
            let error = module
                .call(&panic_case, (value,), &mut Vec::new())
                .unwrap_err()
                .into_materialized();
            let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                panic!("source panic");
            };
            assert_eq!(panic.site().function(), "stop");
            assert_eq!(
                panic.message(),
                &PanicMessage::Explicit("loop callback stop".into())
            );
            panics.push(panic);
        }
        if prepared {
            assert_eq!(panics, canonical_panics);
        } else {
            canonical_panics = panics;
        }
    }
}

#[derive(Default)]
struct LoopTrace {
    allowance: usize,
    entries: usize,
    call_stops: usize,
    logical_steps: usize,
    completed: bool,
}

static TRACE: Mutex<LoopTrace> = Mutex::new(LoopTrace {
    allowance: 1,
    entries: 0,
    call_stops: 0,
    logical_steps: 0,
    completed: false,
});

fn traced_loop(
    point: usize,
    values: &mut data::compiled::custom_loop::CustomLoopValues,
    lists: &data::compiled::custom_loop::CustomListOps<'_>,
    budget: &mut usize,
    boolean: bool,
    index: usize,
) -> data::compiled::custom_loop::CustomLoopProgress {
    use data::compiled::custom_loop::CustomLoopProgress;
    use data::compiled::{CompiledImplementation, CompiledProgress};
    let target = if boolean {
        &LOOP.program.compiled.bools[index].implementation
    } else {
        &LOOP.program.compiled.ints[index].implementation
    };
    let CompiledImplementation::CustomLoop(implementation) = target else {
        panic!("fixture target must be a custom loop");
    };
    let allowance = TRACE.lock().unwrap().allowance.min(*budget);
    let mut remaining = allowance;
    let progress = (implementation.run)(point, values, lists, &mut remaining);
    *budget -= allowance - remaining;
    let mut trace = TRACE.lock().unwrap();
    trace.entries += usize::from(point == implementation.entry);
    trace.call_stops += usize::from(matches!(progress, CustomLoopProgress::Call { .. }));
    trace.logical_steps += allowance - remaining;
    trace.completed |= matches!(
        progress,
        CustomLoopProgress::Caller(CompiledProgress::Complete(_))
    );
    progress
}

fn traced_int(
    point: usize,
    values: &mut data::compiled::custom_loop::CustomLoopValues,
    lists: &data::compiled::custom_loop::CustomListOps<'_>,
    budget: &mut usize,
) -> data::compiled::custom_loop::CustomLoopProgress {
    traced_loop(point, values, lists, budget, false, 0)
}

fn traced_bool(
    point: usize,
    values: &mut data::compiled::custom_loop::CustomLoopValues,
    lists: &data::compiled::custom_loop::CustomListOps<'_>,
    budget: &mut usize,
) -> data::compiled::custom_loop::CustomLoopProgress {
    traced_loop(point, values, lists, budget, true, 0)
}

fn traced_adjusted(
    point: usize,
    values: &mut data::compiled::custom_loop::CustomLoopValues,
    lists: &data::compiled::custom_loop::CustomListOps<'_>,
    budget: &mut usize,
) -> data::compiled::custom_loop::CustomLoopProgress {
    traced_loop(point, values, lists, budget, false, 1)
}

fn traced_guarded(
    point: usize,
    values: &mut data::compiled::custom_loop::CustomLoopValues,
    lists: &data::compiled::custom_loop::CustomListOps<'_>,
    budget: &mut usize,
) -> data::compiled::custom_loop::CustomLoopProgress {
    traced_loop(point, values, lists, budget, false, 2)
}

#[test]
fn generated_caller_resumes_after_interrupted_callbacks_and_small_overflow() {
    use data::compiled::{CompiledFunction, CompiledImplementation, CustomLoopImplementation};
    const BASE: data::ModuleArtifact<Infallible> = include!("fixtures/prepared/custom_loop.rs");
    let mut artifact = BASE;
    assert_eq!(artifact.program.compiled.ints.len(), 4);
    artifact.program.compiled.ints = artifact
        .program
        .compiled
        .ints
        .iter()
        .map(|target| {
            let CompiledImplementation::CustomLoop(implementation) = &target.implementation else {
                panic!("fixture integer loop");
            };
            CompiledFunction {
                function: target.function,
                implementation: CompiledImplementation::CustomLoop(
                    Box::new(CustomLoopImplementation {
                        entry: implementation.entry,
                        checkpoints: implementation.checkpoints.clone(),
                        calls: implementation.calls.clone(),
                        run: if target.function == LOOP.program.compiled.ints[0].function {
                            traced_int
                        } else if target.function == LOOP.program.compiled.ints[1].function {
                            traced_adjusted
                        } else if target.function == LOOP.program.compiled.ints[2].function {
                            traced_guarded
                        } else {
                            implementation.run
                        },
                    })
                    .into(),
                ),
            }
        })
        .collect::<Vec<_>>()
        .into();
    artifact.program.compiled.bools = artifact
        .program
        .compiled
        .bools
        .iter()
        .map(|target| {
            let CompiledImplementation::CustomLoop(implementation) = &target.implementation else {
                panic!("fixture boolean loop");
            };
            CompiledFunction {
                function: target.function,
                implementation: CompiledImplementation::CustomLoop(
                    Box::new(CustomLoopImplementation {
                        entry: implementation.entry,
                        checkpoints: implementation.checkpoints.clone(),
                        calls: implementation.calls.clone(),
                        run: traced_bool,
                    })
                    .into(),
                ),
            }
        })
        .collect::<Vec<_>>()
        .into();
    let artifact = Box::leak(Box::new(artifact));
    let mut bindings = artifact.load().unwrap();
    let run = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new("run"))
        .unwrap();
    let (
        run,
        chosen,
        captured,
        boolean,
        assertion,
        empty,
        panic_case,
        unsupported,
        custom_capture,
        repeated,
        caller_overflow,
        guarded,
        _,
        _,
    ) = functions!(bindings, run);
    let module = bindings.seal();
    let big: BigInt = BigInt::from(1) << 180;
    for allowance in [1, 2, 3, 4, 5, 6, 7, 8, 31, 1024] {
        *TRACE.lock().unwrap() = LoopTrace {
            allowance,
            ..LoopTrace::default()
        };
        assert_eq!(
            module
                .call(&run, (3.into(), 9.into()), &mut Vec::new())
                .unwrap(),
            BigInt::from(10)
        );
        let trace = TRACE.lock().unwrap();
        assert!(trace.entries > 0 && trace.completed);
        if allowance == 1 {
            assert_eq!(trace.call_stops, 4);
        }
        if allowance == 1024 {
            // Four caller iterations (5 steps each), callback branches
            // Add/Subtract/Add/Skip (3/4/3/3), then empty test and return.
            assert_eq!(trace.logical_steps, 35);
            assert_eq!(trace.call_stops, 0);
        }
        drop(trace);
        *TRACE.lock().unwrap() = LoopTrace {
            allowance,
            ..LoopTrace::default()
        };
        assert_eq!(
            module
                .call(&run, (3.into(), big.clone()), &mut Vec::new())
                .unwrap(),
            BigInt::from(10)
        );
        assert!(TRACE.lock().unwrap().completed);
        assert_eq!(
            module
                .call(&run, (i64::MAX.into(), 1.into()), &mut Vec::new())
                .unwrap(),
            BigInt::from(i64::MAX) + 7
        );
        assert_eq!(
            module
                .call(&chosen, (3.into(), true), &mut Vec::new())
                .unwrap(),
            BigInt::from(13)
        );
        assert_eq!(
            module
                .call(&captured, (3.into(), big.clone(), true), &mut Vec::new())
                .unwrap(),
            &big + 8
        );
        assert_eq!(
            module
                .call(&custom_capture, (3.into(), big.clone()), &mut Vec::new())
                .unwrap(),
            &big + 8
        );
        for initial in [false, true] {
            assert_eq!(
                module
                    .call(&boolean, (initial, (-3).into()), &mut Vec::new())
                    .unwrap(),
                initial
            );
        }
        assert_eq!(
            module
                .call(&repeated, (31.into(), 3.into(), 7.into()), &mut Vec::new())
                .unwrap(),
            BigInt::from(100)
        );
        assert_eq!(
            module
                .call(&caller_overflow, (i64::MAX.into(),), &mut Vec::new())
                .unwrap(),
            BigInt::from(i64::MAX) + 3
        );
        *TRACE.lock().unwrap() = LoopTrace {
            allowance,
            ..LoopTrace::default()
        };
        assert_eq!(
            module
                .call(&guarded, (3.into(), 4.into(), true), &mut Vec::new())
                .unwrap(),
            BigInt::from(17)
        );
        let trace = TRACE.lock().unwrap();
        assert!(trace.entries > 0 && trace.completed);
        drop(trace);
        assert_eq!(
            module
                .call(&guarded, (3.into(), 4.into(), false), &mut Vec::new())
                .unwrap(),
            BigInt::from(9)
        );
        assert_eq!(
            module
                .call(&guarded, (3.into(), big.clone(), true), &mut Vec::new())
                .unwrap(),
            BigInt::from(9) + &big * 2
        );
        assert!(
            module
                .call(&assertion, (7.into(),), &mut Vec::new())
                .is_err()
        );
        assert!(
            module
                .call(&panic_case, (7.into(),), &mut Vec::new())
                .is_err()
        );
        *TRACE.lock().unwrap() = LoopTrace {
            allowance,
            ..LoopTrace::default()
        };
        assert_eq!(
            module.call(&empty, (), &mut Vec::new()).unwrap(),
            BigInt::from(19)
        );
        assert_eq!(TRACE.lock().unwrap().call_stops, 0);
        *TRACE.lock().unwrap() = LoopTrace {
            allowance,
            ..LoopTrace::default()
        };
        assert_eq!(
            module
                .call(&unsupported, (7.into(),), &mut Vec::new())
                .unwrap(),
            BigInt::from(8)
        );
        assert_eq!(TRACE.lock().unwrap().entries, 0);
    }
}

#[test]
fn prepared_connection_and_capture_owners_can_move_to_another_thread() {
    let mut bindings = LOOP.load().unwrap();
    let run = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
            "captured",
        ))
        .unwrap();
    let module = bindings.seal();
    let value = std::thread::spawn(move || {
        module
            .call(&run, (3.into(), 4.into(), true), &mut Vec::new())
            .unwrap()
    })
    .join()
    .unwrap();
    assert_eq!(value, BigInt::from(12));
}

#[cfg(feature = "tokio")]
#[test]
fn connected_standalone_entry_runs_without_the_source_compiler() {
    use geam_core::execution::TokioHost;
    use geam_core::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
    use geam_core::{
        HostProviderSet, ModuleSource, PackageSource, PreparedHostedEntry,
        compile_typed_host_program, plan_host_program,
    };
    use work_representation::WorkComponent;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    static ENTRY: data::HostedEntryArtifact = include!("fixtures/prepared/custom_loop_entry.rs");
    let typed = compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/custom_loop.gleam",
                SOURCE,
            )],
        )],
        HostProviderSet::<Profile>::new([]).unwrap(),
    )
    .unwrap();
    let prepared =
        PreparedHostedEntry::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    assert_eq!(
        prepared.emit_rust(),
        include_str!("fixtures/prepared/custom_loop_entry.rs").trim()
    );
    assert!(!ENTRY.program.compiled.callbacks.ints.is_empty());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut entry = ENTRY
        .load(HostProviderSet::<Profile>::new([]).unwrap())
        .unwrap();
    let mut echo = Vec::new();
    assert_eq!(
        runtime
            .block_on(entry.run(&host, &mut (), &mut echo))
            .unwrap(),
        geam_core::ExecutionOutcome::Returned(())
    );
    assert!(echo.is_empty());
}

#[test]
fn caller_suffixes_and_boolean_big_captures_keep_their_original_source_behavior() {
    static BOUNDARIES: data::ModuleArtifact<Infallible> =
        include!("fixtures/prepared/custom_loop_boundaries.rs");
    const SOURCE: &str = include_str!("fixtures/prepared/custom_loop_boundaries.gleam");
    let typed =
        compile_typed_module("example", "src/custom_loop_boundaries.gleam", SOURCE).unwrap();
    let (mut bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/custom_loop_boundaries.rs").trim()
    );
    assert!(!BOUNDARIES.program.compiled.callbacks.bools.is_empty());
    let mut bindings = BOUNDARIES.load().unwrap();
    let integer = bindings
        .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))
        .unwrap();
    let boolean = bindings
        .function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))
        .unwrap();
    let module = bindings.seal();
    for connected in [false, true] {
        for count in [0, 1, 2, 129] {
            for initial in [BigInt::from(3), BigInt::from(i64::MAX)] {
                let mut echo = Vec::new();
                let expected = &initial + count * 3;
                assert_eq!(
                    module
                        .call(
                            &integer,
                            (BigInt::from(count), false, connected, initial),
                            &mut echo
                        )
                        .unwrap(),
                    expected
                );
                assert_eq!(echo.len(), 1);
                assert_eq!(echo[0].value(), &geam_core::Value::Int(expected));
                echo.clear();
                let error = module
                    .call(
                        &integer,
                        (BigInt::from(count), true, connected, BigInt::from(3)),
                        &mut echo,
                    )
                    .unwrap_err();
                assert!(
                    matches!(error, CallError::Execution(ExecutionError::Panic(panic)) if panic.kind() == PanicKind::Panic && panic.site().function() == "walk")
                );
                assert!(echo.is_empty());
            }
        }
    }
    let big: BigInt = BigInt::from(1) << 180;
    for bias in [BigInt::from(-2), BigInt::from(0), big.clone(), -big] {
        assert_eq!(
            module
                .call(&boolean, (bias.clone(),), &mut Vec::new())
                .unwrap(),
            bias + 1 > BigInt::from(0)
        );
    }
}

#[cfg(feature = "tokio")]
#[test]
fn caller_suffixes_cross_the_hosted_quantum_without_replaying_effects_or_errors() {
    use geam_core::execution::TokioHost;
    use geam_core::{HostProviderSet, StatelessHostProfile};
    const BASE: data::HostedModuleArtifact =
        include!("fixtures/prepared/custom_loop_boundaries_hosted.rs");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut canonical_results = Vec::new();
    for compiled in [false, true] {
        let mut artifact = BASE;
        if !compiled {
            artifact.module.program.compiled = data::compiled::CompiledFunctions::interpreted();
        }
        let artifact = Box::leak(Box::new(artifact));
        let mut bindings = artifact
            .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
            .unwrap();
        let integer = bindings
            .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))
            .unwrap();
        let boolean = bindings
            .function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))
            .unwrap();
        let mut module = bindings.seal();
        let mut echo = Vec::new();
        let results = runtime.block_on(module.with_execution(&host, &mut (), &mut echo, async |scope| {
            let mut results = Vec::new();
            // Vary the complete producer and consumer across the ordinary
            // 1024-step hosted quantum, including Small-to-Big caller steps.
            for count in 0..=260 {
                for connected in [false, true] {
                    for initial in [BigInt::from(3), BigInt::from(i64::MAX)] {
                        let value = scope.call(&integer, (BigInt::from(count), false, connected, initial.clone())).await.unwrap();
                        assert_eq!(value, &initial + count * 3);
                        let error = scope.call(&integer, (BigInt::from(count), true, connected, initial)).await.unwrap_err();
                        assert!(matches!(&error, CallError::Execution(ExecutionError::Panic(panic)) if panic.kind() == PanicKind::Panic && panic.site().function() == "walk"));
                        results.push((value, error));
                    }
                }
            }
            let big: BigInt = BigInt::from(1) << 180;
            for bias in [BigInt::from(-2), BigInt::from(0), big.clone(), -big] {
                assert_eq!(scope.call(&boolean, (bias.clone(),)).await.unwrap(), bias + 1 > BigInt::from(0));
            }
            results
        })).unwrap().try_into_value().unwrap();
        assert_eq!(echo.len(), results.len());
        for (output, (expected, _)) in echo.iter().zip(&results) {
            assert_eq!(output.value(), &geam_core::Value::Int(expected.clone()));
        }
        if compiled {
            assert_eq!(results, canonical_results);
        } else {
            canonical_results = results;
        }
    }
}
