use geam_core::__prepared_support as data;

#[path = "fixtures/prepared/string_native_provider.rs"]
mod provider;

const GENERATED: data::HostedModuleArtifact = include!("fixtures/prepared/string_native_calls.rs");
static ARTIFACT: data::HostedModuleArtifact = GENERATED;

#[cfg(feature = "tokio")]
mod bounded_native {
    use super::{ARTIFACT, GENERATED, data};
    use data::compiled::calls::{
        CallExecution, CallInputs, CallOps, CallProgress, CallStart, CallStorage,
        StringNativeExecution,
    };
    use data::compiled::{
        CallTarget, CompiledFunction, CompiledImplementation, FunctionCallsImplementation,
    };
    use geam_core::StringValue;
    use std::sync::Mutex;

    pub(super) static OBSERVATIONS: Mutex<Vec<(usize, usize, &'static str)>> =
        Mutex::new(Vec::new());
    struct Limited(Box<dyn CallExecution>);
    struct NativeDelivery(Box<dyn StringNativeExecution>);

    fn observe(
        execution: Box<dyn CallExecution>,
        ops: &mut CallOps<'_>,
        budget: &mut usize,
    ) -> CallProgress {
        let offered = usize::from(!OBSERVATIONS.lock().unwrap().is_empty()).min(*budget);
        let mut remaining = offered;
        let progress = execution.advance(ops, &mut remaining);
        *budget -= offered - remaining;
        let boundary = match &progress {
            CallProgress::Yield(_) => "yield",
            CallProgress::Complete { .. } => "complete",
            CallProgress::StringNative(_) => "native",
            _ => "canonical",
        };
        OBSERVATIONS
            .lock()
            .unwrap()
            .push((offered, offered - remaining, boundary));
        match progress {
            CallProgress::Yield(next) => CallProgress::Yield(Box::new(Limited(next))),
            CallProgress::StringNative(mut request) => {
                request.execution = Box::new(NativeDelivery(request.execution));
                CallProgress::StringNative(request)
            }
            CallProgress::String {
                function,
                site,
                arguments,
                resume,
            } => CallProgress::String {
                function,
                site,
                arguments,
                resume: Box::new(move |value| Box::new(Limited(resume(value)))),
            },
            CallProgress::InterpretedString {
                function,
                site,
                point,
                values,
                resume,
            } => CallProgress::InterpretedString {
                function,
                site,
                point,
                values,
                resume: Box::new(move |value| Box::new(Limited(resume(value)))),
            },
            progress => progress,
        }
    }
    impl CallExecution for Limited {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            observe(self.0, ops, budget)
        }
    }
    impl CallExecution for NativeDelivery {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            self.0.advance(ops, budget)
        }
    }
    impl StringNativeExecution for NativeDelivery {
        fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
            Box::new(Limited(self.0.resume_native(value)))
        }
    }
    fn start(
        name: &str,
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        let slot = ARTIFACT
            .module
            .exports
            .iter()
            .find(|entry| entry.name.as_ref() == name)
            .unwrap()
            .slot;
        let target = CallTarget::String(ARTIFACT.module.entries.strings[slot].function);
        let row = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .find(|row| row.function == target)
            .unwrap();
        let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
            panic!("selected String wrapper must own generated calls")
        };
        (body.start)(point, inputs, storage)
            .map(|execution| Box::new(Limited(execution)) as Box<dyn CallExecution>)
    }
    fn ordinary(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("ordinary", point, inputs, storage)
    }
    fn tail(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("tail", point, inputs, storage)
    }
    fn nested_tail(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("nested_tail", point, inputs, storage)
    }
    fn after_continuing(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("after_continuing", point, inputs, storage)
    }

    pub(super) fn artifact() -> &'static data::HostedModuleArtifact {
        let mut artifact = GENERATED;
        let replacements: [CallStart; 4] = [ordinary, tail, nested_tail, after_continuing];
        artifact.module.program.compiled.function_calls = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .map(|row| {
                let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
                    panic!("maintained fixture must contain generated calls")
                };
                let replacement = ARTIFACT
                    .module
                    .entries
                    .strings
                    .iter()
                    .take(4)
                    .position(|entry| CallTarget::String(entry.function) == row.function);
                CompiledFunction {
                    function: row.function,
                    implementation: CompiledImplementation::FunctionCalls(
                        Box::new(FunctionCallsImplementation {
                            root: body.root,
                            entry: body.entry,
                            checkpoints: body.checkpoints.clone(),
                            locals: body.locals.clone(),
                            calls: body.calls.clone(),
                            creations: body.creations.clone(),
                            returns: body.returns.clone(),
                            tails: body.tails.clone(),
                            start: replacement.map_or(body.start, |slot| replacements[slot]),
                        })
                        .into(),
                    ),
                }
            })
            .collect();
        Box::leak(Box::new(artifact))
    }
}

#[cfg(feature = "tokio")]
#[test]
fn actual_generated_string_native_calls_resume_through_zero_and_single_instruction_allowances() {
    use geam_core::embedding::{FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for continuing in [false, true] {
        for (name, expected, effects, native_count, canonical_count) in [
            ("ordinary", "input<1>2", 2, 2, 0),
            ("tail", "input<3", 1, 1, 0),
            ("nested_tail", "input<4>5", 2, 2, 0),
            ("after_continuing", "input<6>7", 2, 2, 1),
        ] {
            let mut bindings = bounded_native::artifact()
                .load(provider::hosts(continuing))
                .unwrap();
            let function = bindings
                .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                    name,
                ))
                .unwrap();
            let mut module = bindings.seal();
            let mut audit = provider::Audit::default();
            bounded_native::OBSERVATIONS.lock().unwrap().clear();
            let value = runtime
                .block_on(module.with_execution(
                    &host,
                    &mut audit,
                    &mut Vec::new(),
                    async |scope| scope.call(&function, ("input".into(),)).await,
                ))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(value.as_str().unwrap(), expected);
            assert_eq!(audit.events.len(), effects);
            let observations = bounded_native::OBSERVATIONS.lock().unwrap();
            assert_eq!(observations[0], (0, 0, "yield"));
            assert!(
                observations
                    .iter()
                    .all(|(offered, consumed, _)| *offered <= 1 && consumed <= offered)
            );
            assert_eq!(
                observations
                    .iter()
                    .filter(|(_, _, boundary)| *boundary == "native")
                    .count(),
                if continuing { 0 } else { native_count },
                "{name}/{continuing}"
            );
            assert_eq!(
                observations
                    .iter()
                    .filter(|(_, _, boundary)| *boundary == "canonical")
                    .count(),
                if continuing {
                    native_count + canonical_count
                } else {
                    canonical_count
                },
                "{name}/{continuing}"
            );
            let final_boundary = if continuing && name == "tail" {
                "canonical"
            } else {
                "complete"
            };
            assert_eq!(
                observations.last().unwrap().2,
                final_boundary,
                "{name}/{continuing}"
            );
        }
    }
}

#[test]
fn generated_string_calls_match_the_maintained_preparation() {
    assert_eq!(
        provider::prepare().emit_rust().trim(),
        include_str!("fixtures/prepared/string_native_calls.rs").trim()
    );
}

#[cfg(feature = "tokio")]
#[test]
fn synchronous_and_continuing_calls_preserve_nullary_arguments_results_and_effect_order() {
    use geam_core::embedding::{FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        for continuing in [false, true] {
            for (name, expected, indices, continued) in [
                ("ordinary", "input<1>2", vec![1, 2], 0),
                ("tail", "input<3", vec![3], 0),
                ("nested_tail", "input<4>5", vec![4, 5], 0),
                ("after_continuing", "input<6>7", vec![6, 7], 1),
            ] {
                let (mut module, function) = if prepared {
                    let mut bindings = ARTIFACT.load(provider::hosts(continuing)).unwrap();
                    let function = bindings
                        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                            name,
                        ))
                        .unwrap();
                    (bindings.seal(), function)
                } else {
                    let (bindings, function) = provider::source(continuing)
                        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                            name,
                        ))
                        .unwrap();
                    (bindings.seal().unwrap(), function)
                };
                let mut audit = provider::Audit::default();
                let mut echo = Vec::new();
                let result = runtime
                    .block_on(
                        module.with_execution(&host, &mut audit, &mut echo, async |scope| {
                            scope.call(&function, ("input".into(),)).await
                        }),
                    )
                    .unwrap()
                    .try_into_value()
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    result.as_str().unwrap(),
                    expected,
                    "{name}/{prepared}/{continuing}"
                );
                assert_eq!(
                    audit
                        .events
                        .iter()
                        .map(|event| event.2.clone())
                        .collect::<Vec<_>>(),
                    indices.into_iter().map(Into::into).collect::<Vec<_>>()
                );
                assert_eq!(audit.events[0].0.as_str().unwrap(), "input");
                assert!(audit.events[0].1);
                if audit.events.len() == 2 {
                    assert!(!audit.events[1].1);
                }
                assert_eq!(audit.continued, continued);
                assert_eq!(audit.stopped, 0);
                assert!(echo.is_empty());
            }
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn native_failure_cancellation_and_exit_do_not_replay_or_run_later_effects() {
    use geam_core::embedding::{CallError, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut failures = Vec::new();
    for prepared in [false, true] {
        for continuing in [false, true] {
            for (failure, cancelled, exited, name, expected_calls) in [
                (Some(1), None, None, "ordinary", 1),
                (Some(2), None, None, "ordinary", 2),
                (None, Some(1), None, "ordinary", 1),
                (None, Some(2), None, "ordinary", 2),
                (None, None, Some(1), "ordinary", 1),
                (None, None, Some(2), "ordinary", 2),
                (None, None, None, "after_never", 1),
            ] {
                let (mut module, function) = if prepared {
                    let mut bindings = ARTIFACT.load(provider::hosts(continuing)).unwrap();
                    let function = bindings
                        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                            name,
                        ))
                        .unwrap();
                    (bindings.seal(), function)
                } else {
                    let (bindings, function) = provider::source(continuing)
                        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                            name,
                        ))
                        .unwrap();
                    (bindings.seal().unwrap(), function)
                };
                let mut audit = provider::Audit {
                    fail_at: failure,
                    cancel_at: cancelled,
                    exit_at: exited,
                    ..Default::default()
                };
                let result = runtime
                    .block_on(module.with_execution(
                        &host,
                        &mut audit,
                        &mut Vec::new(),
                        async |scope| scope.call(&function, ("input".into(),)).await,
                    ))
                    .unwrap();
                if failure.is_some() {
                    let Err(CallError::Execution(error)) = result.try_into_value().unwrap() else {
                        panic!("native failure must preserve its execution error");
                    };
                    failures.push(error.to_string());
                } else if cancelled.is_some() {
                    assert_eq!(result.try_into_value().unwrap(), Err(CallError::Cancelled));
                } else {
                    assert_eq!(
                        result.try_into_value().unwrap_err().code(),
                        if name == "after_never" { 11 } else { 7 }
                    );
                }
                assert_eq!(
                    audit.events.len(),
                    expected_calls,
                    "{name}/{prepared}/{continuing}"
                );
                assert_eq!(audit.stopped, usize::from(name == "after_never"));
            }
        }
    }
    for pair in failures.chunks_exact(2) {
        assert!(
            pair.iter()
                .all(|failure| failure.contains("string native failure"))
        );
    }
    assert_eq!(&failures[..4], &failures[4..]);
}

#[cfg(feature = "tokio")]
#[test]
fn a_generated_native_completion_keeps_the_callers_return_family_boundary() {
    use data::compiled::calls::{
        CallExecution, CallInputs, CallOps, CallOutput, CallProgress, CallStorage,
        StringNativeExecution,
    };
    use data::compiled::{
        CallTarget, CompiledFunction, CompiledImplementation, FunctionCallsImplementation,
    };
    use geam_core::embedding::{FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;

    // Metadata and the source graph stay unchanged. A Rust execution sidecar
    // can still publish the wrong completion family after real Native effects.
    struct WrongCompletion(Box<dyn CallExecution>);
    struct NativeDelivery(Box<dyn StringNativeExecution>);
    impl CallExecution for WrongCompletion {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            match self.0.advance(ops, budget) {
                CallProgress::Complete { execution, .. } => CallProgress::Complete {
                    output: CallOutput::Bool(true),
                    execution,
                },
                CallProgress::Yield(next) => CallProgress::Yield(Box::new(Self(next))),
                CallProgress::StringNative(mut request) => {
                    request.execution = Box::new(NativeDelivery(request.execution));
                    CallProgress::StringNative(request)
                }
                progress => progress,
            }
        }
    }
    impl CallExecution for NativeDelivery {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            self.0.advance(ops, budget)
        }
    }
    impl StringNativeExecution for NativeDelivery {
        fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
            Box::new(WrongCompletion(self.0.resume_native(value)))
        }
    }
    fn start(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        let slot = ARTIFACT
            .module
            .exports
            .iter()
            .find(|entry| entry.name.as_ref() == "ordinary")
            .unwrap()
            .slot;
        let target = CallTarget::String(ARTIFACT.module.entries.strings[slot].function);
        let row = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .find(|row| row.function == target)
            .unwrap();
        let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
            panic!("ordinary source owns generated calls")
        };
        (body.start)(point, inputs, storage)
            .map(|execution| Box::new(WrongCompletion(execution)) as Box<dyn CallExecution>)
    }
    let slot = ARTIFACT
        .module
        .exports
        .iter()
        .find(|entry| entry.name.as_ref() == "ordinary")
        .unwrap()
        .slot;
    let target = CallTarget::String(ARTIFACT.module.entries.strings[slot].function);
    let mut artifact = GENERATED;
    artifact.module.program.compiled.function_calls = ARTIFACT
        .module
        .program
        .compiled
        .function_calls
        .iter()
        .map(|row| {
            let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
                panic!("function_calls contains generated call implementations")
            };
            CompiledFunction {
                function: row.function,
                implementation: CompiledImplementation::FunctionCalls(
                    Box::new(FunctionCallsImplementation {
                        root: body.root,
                        entry: body.entry,
                        checkpoints: body.checkpoints.clone(),
                        locals: body.locals.clone(),
                        calls: body.calls.clone(),
                        creations: body.creations.clone(),
                        returns: body.returns.clone(),
                        tails: body.tails.clone(),
                        start: if row.function == target {
                            start
                        } else {
                            body.start
                        },
                    })
                    .into(),
                ),
            }
        })
        .collect();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut bindings = Box::leak(Box::new(artifact))
        .load(provider::hosts(false))
        .unwrap();
    let caller = bindings
        .function(FunctionDeclaration::<
            (StringValue,),
            (StringValue, StringValue),
        >::new("source_caller"))
        .unwrap();
    let string_caller = bindings
        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
            "source_string_caller",
        ))
        .unwrap();
    let mut module = bindings.seal();
    let mut audit = provider::Audit::default();
    let mut echo = Vec::new();
    let result = runtime
        .block_on(
            module.with_execution(&host, &mut audit, &mut echo, async |scope| {
                scope.call(&caller, (StringValue::from("input"),)).await
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
    assert_eq!(
        result.unwrap_err().to_string(),
        "function return family mismatch (expected String, got Bool)"
    );
    assert_eq!(
        audit.events,
        [
            (StringValue::from("input"), true, 1.into()),
            (StringValue::from("input<1"), false, 2.into())
        ]
    );
    assert_eq!(echo.len(), 1);
    assert_eq!(echo[0].value().inspect().to_string(), "\"input\"");
    // A String caller must stop before it concatenates the normal result.
    audit = provider::Audit::default();
    echo.clear();
    let result = runtime
        .block_on(
            module.with_execution(&host, &mut audit, &mut echo, async |scope| {
                scope
                    .call(&string_caller, (StringValue::from("input"),))
                    .await
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
    assert_eq!(
        result.unwrap_err().to_string(),
        "function return family mismatch (expected String, got Bool)"
    );
    assert_eq!(
        audit.events,
        [
            (StringValue::from("input"), true, 1.into()),
            (StringValue::from("input<1"), false, 2.into())
        ]
    );
    assert_eq!(echo.len(), 1);
    assert_eq!(echo[0].value().inspect().to_string(), "\"input\"");
}
