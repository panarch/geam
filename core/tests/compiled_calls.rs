use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallProgress, CallStorage};
use data::compiled::{
    CallTarget, CompiledFunction, CompiledImplementation, FunctionCallsImplementation,
};
use geam_core::__prepared_support as data;
use geam_core::compile_typed_module;
use geam_core::embedding::{BigInt, CallError, FunctionDeclaration, ModuleBuilder};
use geam_core::{ExecutionError, PanicKind, PanicMessage};
use std::convert::Infallible;
use std::sync::Mutex;

static CALLS: data::ModuleArtifact<Infallible> = include!("fixtures/prepared/function_calls.rs");

static BOOLEAN_CALLS: data::ModuleArtifact<Infallible> =
    include!("fixtures/prepared/boolean_calls.rs");

#[test]
fn generated_boolean_only_calls_compile_without_unused_step_variants_and_return_both_values() {
    let source = include_str!("fixtures/prepared/boolean_calls.gleam");
    let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(bool,), bool>::new("flip"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/boolean_calls.rs").trim()
    );
    assert!(!BOOLEAN_CALLS.program.compiled.function_calls.is_empty());
    let mut bindings = BOOLEAN_CALLS.load().unwrap();
    let compiled_flip = bindings
        .function(FunctionDeclaration::<(bool,), bool>::new("flip"))
        .unwrap();
    let prepared = bindings.seal();
    let mut echo = Vec::new();
    for (input, expected) in [(false, true), (true, false)] {
        assert_eq!(
            prepared.call(&compiled_flip, (input,), &mut echo).unwrap(),
            expected
        );
    }
    assert!(echo.is_empty());
}

static BOOLEAN_BRIDGE: data::ModuleArtifact<Infallible> =
    include!("fixtures/prepared/boolean_bridge.rs");

#[test]
fn terminal_boolean_bridges_match_live_calls_and_keep_stable_generated_data() {
    let source = include_str!("fixtures/prepared/boolean_bridge.gleam");
    let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), bool>::new("verify"))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/boolean_bridge.rs").trim()
    );
    let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (bindings, live_verify) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), bool>::new("verify"))
        .unwrap();
    let mut prepared = BOOLEAN_BRIDGE.load().unwrap();
    let prepared_verify = prepared
        .function(FunctionDeclaration::<(), bool>::new("verify"))
        .unwrap();
    for (module, verify) in [
        (bindings.seal(), live_verify),
        (prepared.seal(), prepared_verify),
    ] {
        let mut echo = Vec::new();
        for _ in 0..3 {
            assert!(module.call(&verify, (), &mut echo).unwrap());
        }
        assert!(echo.is_empty());
    }
}

static BRIDGE_TRACE: Mutex<Vec<(usize, usize, &'static str)>> = Mutex::new(Vec::new());

struct LimitedBridge(Box<dyn CallExecution>);

impl CallExecution for LimitedBridge {
    fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
        self.0.restart(target, point, inputs)
    }

    fn retained_bytes(&self) -> usize {
        self.0.retained_bytes()
    }

    fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
        let offered = if BRIDGE_TRACE.lock().unwrap().is_empty() {
            0
        } else {
            1.min(*budget)
        };
        let mut remaining = offered;
        let progress = self.0.advance(ops, &mut remaining);
        let consumed = offered - remaining;
        *budget -= consumed;
        let route = match &progress {
            CallProgress::Yield(_) => "yield",
            CallProgress::Bool { .. } => "bridge",
            CallProgress::Interpreted { .. } => "canonical",
            _ => "other",
        };
        BRIDGE_TRACE
            .lock()
            .unwrap()
            .push((offered, consumed, route));
        match progress {
            CallProgress::Yield(next) => CallProgress::Yield(Box::new(Self(next))),
            CallProgress::Bool {
                function,
                site,
                arguments,
                resume,
            } => CallProgress::Bool {
                function,
                site,
                arguments,
                resume: Box::new(move |value| Box::new(Self(resume(value)))),
            },
            progress => progress,
        }
    }
}

fn limited_boolean_bridge(
    point: usize,
    inputs: CallInputs<'_>,
    storage: &mut CallStorage,
) -> Option<Box<dyn CallExecution>> {
    let row = BOOLEAN_BRIDGE.program.compiled.function_calls.first()?;
    let CompiledImplementation::FunctionCalls(implementation) = &row.implementation else {
        return None;
    };
    (implementation.start)(point, inputs, storage)
        .map(|execution| Box::new(LimitedBridge(execution)) as Box<dyn CallExecution>)
}

#[test]
fn terminal_bridges_preserve_zero_budget_yields_and_single_step_resumption() {
    const BASE: data::ModuleArtifact<Infallible> = include!("fixtures/prepared/boolean_bridge.rs");
    let mut artifact = BASE;
    let original_rows = artifact.program.compiled.function_calls.len();
    assert_eq!(original_rows, 1);
    artifact.program.compiled.function_calls = artifact
        .program
        .compiled
        .function_calls
        .iter()
        .filter_map(|row| {
            let CompiledImplementation::FunctionCalls(implementation) = &row.implementation else {
                return None;
            };
            Some(CompiledFunction {
                function: row.function,
                implementation: CompiledImplementation::FunctionCalls(
                    Box::new(FunctionCallsImplementation {
                        root: implementation.root,
                        entry: implementation.entry,
                        checkpoints: implementation.checkpoints.clone(),
                        locals: implementation.locals.clone(),
                        calls: implementation.calls.clone(),
                        creations: implementation.creations.clone(),
                        returns: implementation.returns.clone(),
                        tails: implementation.tails.clone(),
                        start: limited_boolean_bridge,
                    })
                    .into(),
                ),
            })
        })
        .collect::<Vec<_>>()
        .into();
    assert_eq!(
        artifact.program.compiled.function_calls.len(),
        original_rows
    );
    let artifact = Box::leak(Box::new(artifact));
    let mut bindings = artifact.load().unwrap();
    let verify = bindings
        .function(FunctionDeclaration::<(), bool>::new("verify"))
        .unwrap();
    let module = bindings.seal();
    for _ in 0..2 {
        BRIDGE_TRACE.lock().unwrap().clear();
        assert!(module.call(&verify, (), &mut Vec::new()).unwrap());
        assert_eq!(
            BRIDGE_TRACE.lock().unwrap().as_slice(),
            [(0, 0, "yield"), (1, 1, "bridge"), (1, 0, "canonical")]
        );
    }
}

macro_rules! select_remaining {
    ($bindings:ident, $chain:expr) => {{
        let target = $bindings
            .function(FunctionDeclaration::<(bool, BigInt), BigInt>::new(
                "dynamic_target",
            ))
            .unwrap();
        let nested = $bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("nested"))
            .unwrap();
        let mutual = $bindings
            .function(FunctionDeclaration::<(BigInt,), bool>::new("mutual"))
            .unwrap();
        let captures = $bindings
            .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
                "callable_captures",
            ))
            .unwrap();
        let aliases = $bindings
            .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
                "aliases",
            ))
            .unwrap();
        let canonical = $bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("canonical"))
            .unwrap();
        let big = $bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("big_return"))
            .unwrap();
        let failure = $bindings
            .function(FunctionDeclaration::<(BigInt,), BigInt>::new("failure"))
            .unwrap();
        (
            $bindings.seal(),
            $chain,
            target,
            nested,
            mutual,
            captures,
            aliases,
            canonical,
            big,
            failure,
        )
    }};
}

struct CallCharge {
    offered: usize,
    consumed: usize,
    complete: bool,
}

struct CallTrace {
    allowance: usize,
    charges: Vec<CallCharge>,
}

static CALL_TRACE: Mutex<CallTrace> = Mutex::new(CallTrace {
    allowance: 1,
    charges: Vec::new(),
});

struct LimitedCalls(Box<dyn CallExecution>);

impl CallExecution for LimitedCalls {
    fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
        self.0.restart(target, point, inputs)
    }

    fn retained_bytes(&self) -> usize {
        self.0.retained_bytes()
    }

    fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
        let offered = CALL_TRACE.lock().unwrap().allowance.min(*budget);
        let mut remaining = offered;
        let progress = self.0.advance(ops, &mut remaining);
        let consumed = offered - remaining;
        *budget -= consumed;
        let complete = matches!(&progress, CallProgress::Complete { .. });
        CALL_TRACE.lock().unwrap().charges.push(CallCharge {
            offered,
            consumed,
            complete,
        });
        match progress {
            CallProgress::Yield(next) => CallProgress::Yield(Box::new(Self(next))),
            complete @ CallProgress::Complete { .. } => complete,
            _ => panic!("Small capture chains must stay within the actual generated execution"),
        }
    }
}

fn limited_capture_chain(
    point: usize,
    values: CallInputs<'_>,
    storage: &mut CallStorage,
) -> Option<Box<dyn CallExecution>> {
    let target = CallTarget::Int(CALLS.entries.ints[0].function);
    let row = CALLS
        .program
        .compiled
        .function_calls
        .iter()
        .find(|row| row.function == target)
        .unwrap();
    let CompiledImplementation::FunctionCalls(implementation) = &row.implementation else {
        panic!("the selected capture_chain must have its actual generated entry");
    };
    (implementation.start)(point, values, storage)
        .map(|execution| Box::new(LimitedCalls(execution)) as Box<dyn CallExecution>)
}

#[test]
fn actual_generated_calls_preserve_every_charged_return_at_small_budget_boundaries() {
    const BASE: data::ModuleArtifact<Infallible> = include!("fixtures/prepared/function_calls.rs");
    let mut artifact = BASE;
    let target = CallTarget::Int(artifact.entries.ints[0].function);
    artifact.program.compiled.function_calls = artifact
        .program
        .compiled
        .function_calls
        .iter()
        .map(|row| {
            let CompiledImplementation::FunctionCalls(implementation) = &row.implementation else {
                panic!("call sidecar rows must be function-call implementations");
            };
            CompiledFunction {
                function: row.function,
                implementation: CompiledImplementation::FunctionCalls(
                    Box::new(FunctionCallsImplementation {
                        root: implementation.root,
                        entry: implementation.entry,
                        checkpoints: implementation.checkpoints.clone(),
                        locals: implementation.locals.clone(),
                        calls: implementation.calls.clone(),
                        creations: implementation.creations.clone(),
                        returns: implementation.returns.clone(),
                        tails: implementation.tails.clone(),
                        start: if row.function == target {
                            limited_capture_chain
                        } else {
                            implementation.start
                        },
                    })
                    .into(),
                ),
            }
        })
        .collect::<Vec<_>>()
        .into();
    let artifact = Box::leak(Box::new(artifact));
    let mut bindings = artifact.load().unwrap();
    let chain = bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "capture_chain",
        ))
        .unwrap();
    let module = bindings.seal();
    // Comparison and branch are one canonical TestBranch. The root/identity
    // account for eight steps: three root steps, two compose steps,
    // two terminal chain steps and one identity Return. Each layer adds four producer steps and
    // Call/Add/Return: seven steps. Graph Return resumes its caller in that step.
    for depth in [0, 1, 8, 800] {
        for allowance in (1..=8).chain([31, 1024]) {
            *CALL_TRACE.lock().unwrap() = CallTrace {
                allowance,
                charges: Vec::new(),
            };
            assert_eq!(
                module
                    .call(&chain, (depth.into(), 7.into()), &mut Vec::new())
                    .unwrap(),
                BigInt::from(depth + 7)
            );
            let trace = CALL_TRACE.lock().unwrap();
            let expected_steps = 8 + 7 * depth as usize;
            let mut completed_steps = 0;
            for charge in &trace.charges {
                let expected = charge.offered.min(expected_steps - completed_steps);
                assert_eq!(
                    charge.consumed, expected,
                    "depth {depth}, allowance {allowance}, prefix {completed_steps}"
                );
                completed_steps += expected;
                assert_eq!(
                    charge.complete,
                    completed_steps == expected_steps,
                    "depth {depth}, allowance {allowance}, prefix {completed_steps}"
                );
            }
            assert_eq!(completed_steps, expected_steps);
            assert!(trace.charges.last().unwrap().complete);
        }
    }
}

#[test]
fn generated_function_calls_cover_production_capture_and_nested_return_paths() {
    let source = include_str!("fixtures/prepared/function_calls.gleam");
    let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
    let (mut bindings, _) = ModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "capture_chain",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(bool, BigInt), BigInt>::new(
            "dynamic_target",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("nested"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), bool>::new("mutual"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
            "callable_captures",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
            "aliases",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("canonical"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("big_return"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("failure"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "producer_suffix_int",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
            "producer_suffix_bool",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "reuse_callback",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt, BigInt, BigInt), BigInt>::new("repeated_roots"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), bool>::new(
            "canonical_bool",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(bool, BigInt, BigInt), bool>::new(
            "bool_captures",
        ))
        .unwrap();
    assert_eq!(
        bindings.prepare().emit_rust(),
        include_str!("fixtures/prepared/function_calls.rs").trim()
    );
    assert!(!CALLS.program.compiled.function_calls.is_empty());

    for prepared in [false, true] {
        let (module, chain, target, nested, mutual, captures, aliases, canonical, big, failure) =
            if prepared {
                let mut bindings = CALLS.load().unwrap();
                let chain = bindings
                    .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                        "capture_chain",
                    ))
                    .unwrap();
                select_remaining!(bindings, chain)
            } else {
                let (mut bindings, chain) = ModuleBuilder::new(
                    compile_typed_module("example", "src/example.gleam", source).unwrap(),
                )
                .unwrap()
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "capture_chain",
                ))
                .unwrap();
                select_remaining!(bindings, chain)
            };
        let mut echo = Vec::new();
        for depth in [0, 1, 8, 800] {
            assert_eq!(
                module
                    .call(&chain, (depth.into(), 7.into()), &mut echo)
                    .unwrap(),
                BigInt::from(depth + 7)
            );
        }
        assert_eq!(
            module.call(&target, (true, 9.into()), &mut echo).unwrap(),
            BigInt::from(12)
        );
        assert_eq!(
            module.call(&target, (false, 9.into()), &mut echo).unwrap(),
            BigInt::from(19)
        );
        assert_eq!(
            module.call(&nested, (20000.into(),), &mut echo).unwrap(),
            BigInt::from(20002)
        );
        assert!(!module.call(&mutual, (10000.into(),), &mut echo).unwrap());
        assert!(module.call(&mutual, (10001.into(),), &mut echo).unwrap());
        assert!(
            !module
                .call(&captures, (3.into(), 7.into()), &mut echo)
                .unwrap()
        );
        assert!(
            module
                .call(&captures, (7.into(), 3.into()), &mut echo)
                .unwrap()
        );
        assert!(
            module
                .call(&aliases, (3.into(), 7.into()), &mut echo)
                .unwrap()
        );
        assert_eq!(
            module.call(&canonical, (7.into(),), &mut echo).unwrap(),
            BigInt::from(10)
        );
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["example::echo_value@1630..1640\n7"]
        );
        echo.clear();
        let value = BigInt::from(i64::MAX);
        assert_eq!(
            module.call(&big, (value.clone(),), &mut echo).unwrap(),
            &value * &value + 3
        );
        let value: BigInt = BigInt::from(1) << 100;
        assert_eq!(
            module.call(&big, (value.clone(),), &mut echo).unwrap(),
            &value * &value + 3
        );
        let error = module.call(&failure, (7.into(),), &mut echo).unwrap_err();
        let CallError::Execution(ExecutionError::Panic(panic)) = error.into_materialized() else {
            panic!("expected the source call suffix failure");
        };
        assert_eq!(panic.kind(), PanicKind::Panic);
        assert_eq!(
            panic.message(),
            &PanicMessage::Explicit("call suffix".into())
        );
        assert_eq!(panic.site().module(), "example");
        assert_eq!(panic.site().function(), "failing");
        assert_eq!(
            echo.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["example::failing@1896..1907\n8"]
        );
    }
}

#[test]
fn generated_producers_resume_canonical_suffixes_and_return_each_callable_family_once() {
    for prepared in [false, true] {
        let (module, integer, boolean) = if prepared {
            let mut bindings = CALLS.load().unwrap();
            let integer = bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "producer_suffix_int",
                ))
                .unwrap();
            let boolean = bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
                    "producer_suffix_bool",
                ))
                .unwrap();
            (bindings.seal(), integer, boolean)
        } else {
            let typed = compile_typed_module(
                "example",
                "src/example.gleam",
                include_str!("fixtures/prepared/function_calls.gleam"),
            )
            .unwrap();
            let (mut bindings, integer) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "producer_suffix_int",
                ))
                .unwrap();
            let boolean = bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
                    "producer_suffix_bool",
                ))
                .unwrap();
            (bindings.seal(), integer, boolean)
        };
        let mut echo = Vec::new();
        assert_eq!(
            module
                .call(&integer, (7.into(), 10.into()), &mut echo)
                .unwrap(),
            BigInt::from(20)
        );
        assert_eq!(
            echo.iter()
                .map(|event| event.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["8"]
        );
        echo.clear();
        assert!(
            !module
                .call(&boolean, (7.into(), 10.into()), &mut echo)
                .unwrap()
        );
        assert_eq!(
            echo.iter()
                .map(|event| event.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["8"]
        );
    }
}

#[test]
fn repeated_completed_roots_keep_fresh_captures_and_reject_big_entries_without_losing_values() {
    let source = include_str!("fixtures/prepared/function_calls.gleam");
    for prepared in [false, true] {
        let (module, repeat) = if prepared {
            let mut bindings = CALLS.load().unwrap();
            let repeat = bindings
                .function(
                    FunctionDeclaration::<(BigInt, BigInt, BigInt), BigInt>::new("repeated_roots"),
                )
                .unwrap();
            (bindings.seal(), repeat)
        } else {
            let (bindings, repeat) = ModuleBuilder::new(
                compile_typed_module("example", "src/example.gleam", source).unwrap(),
            )
            .unwrap()
            .function(
                FunctionDeclaration::<(BigInt, BigInt, BigInt), BigInt>::new("repeated_roots"),
            )
            .unwrap();
            (bindings.seal(), repeat)
        };
        let mut echo = Vec::new();
        for (count, initial, offset, expected) in
            [(100, 7, 3, 5357), (123, 7, -9, 6526), (0, 17, 2, 17)]
        {
            assert_eq!(
                module
                    .call(
                        &repeat,
                        (count.into(), initial.into(), offset.into()),
                        &mut echo
                    )
                    .unwrap(),
                BigInt::from(expected)
            );
        }
        let big: BigInt = BigInt::from(1) << 100;
        assert_eq!(
            module
                .call(&repeat, (100.into(), big.clone(), 3.into()), &mut echo)
                .unwrap(),
            big + 5350
        );
        assert_eq!(
            module
                .call(&repeat, (3.into(), 11.into(), 7.into()), &mut echo)
                .unwrap(),
            BigInt::from(38)
        );
        assert!(echo.is_empty());
    }
}

#[test]
fn generated_boolean_calls_resume_once_and_keep_each_scalar_capture() {
    for prepared in [false, true] {
        let (module, suffix, captures) = if prepared {
            let mut bindings = CALLS.load().unwrap();
            let suffix = bindings
                .function(FunctionDeclaration::<(BigInt,), bool>::new(
                    "canonical_bool",
                ))
                .unwrap();
            let captures = bindings
                .function(FunctionDeclaration::<(bool, BigInt, BigInt), bool>::new(
                    "bool_captures",
                ))
                .unwrap();
            (bindings.seal(), suffix, captures)
        } else {
            let typed = compile_typed_module(
                "example",
                "src/example.gleam",
                include_str!("fixtures/prepared/function_calls.gleam"),
            )
            .unwrap();
            let (mut bindings, suffix) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(BigInt,), bool>::new(
                    "canonical_bool",
                ))
                .unwrap();
            let captures = bindings
                .function(FunctionDeclaration::<(bool, BigInt, BigInt), bool>::new(
                    "bool_captures",
                ))
                .unwrap();
            (bindings.seal(), suffix, captures)
        };
        let mut echo = Vec::new();
        for (value, expected, echoed) in [(7, false, "8"), (-2, true, "-1")] {
            assert_eq!(
                module.call(&suffix, (value.into(),), &mut echo).unwrap(),
                expected
            );
            assert_eq!(
                echo.iter()
                    .map(|event| event.value().inspect().to_string())
                    .collect::<Vec<_>>(),
                [echoed]
            );
            echo.clear();
        }
        for (enabled, offset, value, expected) in [
            (true, 7, 8, true),
            (true, 7, 6, false),
            (false, 7, 8, false),
        ] {
            assert_eq!(
                module
                    .call(&captures, (enabled, offset.into(), value.into()), &mut echo)
                    .unwrap(),
                expected
            );
        }
        assert!(echo.is_empty());
    }
}
