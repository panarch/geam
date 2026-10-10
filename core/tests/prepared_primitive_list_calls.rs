use geam_core::__prepared_support as data;
#[cfg(feature = "tokio")]
use geam_core::embedding::{
    BigInt, BitArrayValue, CallableType, FunctionDeclaration, List, StringValue,
};
#[cfg(feature = "tokio")]
use geam_core::{HostProviderSet, StatelessHostProfile};

#[path = "fixtures/prepared/primitive_list_declarations.rs"]
mod declarations;

const GENERATED_ARTIFACT: data::HostedModuleArtifact =
    include!("fixtures/prepared/primitive_list_calls.rs");
static ARTIFACT: data::HostedModuleArtifact = GENERATED_ARTIFACT;

#[cfg(feature = "tokio")]
mod bounded_execution {
    use super::{ARTIFACT, GENERATED_ARTIFACT};
    use data::compiled::calls::{
        CallExecution, CallInputs, CallOps, CallProgress, CallStart, CallStorage,
    };
    use data::compiled::{
        CallTarget, CompiledFunction, CompiledImplementation, FunctionCallsImplementation,
    };
    use geam_core::__prepared_support as data;
    use std::sync::Mutex;

    #[derive(Default)]
    struct Charge {
        offered: usize,
        consumed: usize,
        complete: bool,
        canonical: bool,
    }

    static CHARGES: Mutex<Vec<Charge>> = Mutex::new(Vec::new());

    struct Limited(Box<dyn CallExecution>);

    impl CallExecution for Limited {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }

        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }

        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            // Offer zero once, then one shared instruction across the entire
            // traversal and its nested callbacks, without resetting on calls.
            let offered = usize::from(!CHARGES.lock().unwrap().is_empty()).min(*budget);
            let mut remaining = offered;
            let progress = self.0.advance(ops, &mut remaining);
            let consumed = offered - remaining;
            *budget -= consumed;
            CHARGES.lock().unwrap().push(Charge {
                offered,
                consumed,
                complete: matches!(&progress, CallProgress::Complete { .. }),
                canonical: !matches!(
                    &progress,
                    CallProgress::Yield(_) | CallProgress::Complete { .. }
                ),
            });
            match progress {
                CallProgress::Yield(next) => CallProgress::Yield(Box::new(Self(next))),
                progress => progress,
            }
        }
    }

    fn start(
        root: CallTarget,
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        let row = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .find(|row| row.function == root)
            .unwrap();
        let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
            panic!("the selected primitive root must own generated calls");
        };
        (body.start)(point, inputs, storage)
            .map(|execution| Box::new(Limited(execution)) as Box<dyn CallExecution>)
    }

    fn integers(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::Int(ARTIFACT.module.entries.ints[0].function),
            point,
            inputs,
            storage,
        )
    }

    fn floats(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::Float(ARTIFACT.module.entries.floats[0].function),
            point,
            inputs,
            storage,
        )
    }

    fn booleans(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::Bool(ARTIFACT.module.entries.bools[0].function),
            point,
            inputs,
            storage,
        )
    }

    fn strings(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::String(ARTIFACT.module.entries.strings[0].function),
            point,
            inputs,
            storage,
        )
    }

    fn bit_arrays(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::BitArray(ARTIFACT.module.entries.bit_arrays[0].function),
            point,
            inputs,
            storage,
        )
    }

    fn codepoints(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::UtfCodepoint(ARTIFACT.module.entries.utf_codepoints[0].function),
            point,
            inputs,
            storage,
        )
    }

    fn nils(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(
            CallTarget::Nil(ARTIFACT.module.entries.nils[0].function),
            point,
            inputs,
            storage,
        )
    }

    pub(super) fn artifact() -> &'static data::HostedModuleArtifact {
        let replacements: [(CallTarget, CallStart); 7] = [
            (
                CallTarget::Int(ARTIFACT.module.entries.ints[0].function),
                integers,
            ),
            (
                CallTarget::Float(ARTIFACT.module.entries.floats[0].function),
                floats,
            ),
            (
                CallTarget::Bool(ARTIFACT.module.entries.bools[0].function),
                booleans,
            ),
            (
                CallTarget::String(ARTIFACT.module.entries.strings[0].function),
                strings,
            ),
            (
                CallTarget::BitArray(ARTIFACT.module.entries.bit_arrays[0].function),
                bit_arrays,
            ),
            (
                CallTarget::UtfCodepoint(ARTIFACT.module.entries.utf_codepoints[0].function),
                codepoints,
            ),
            (
                CallTarget::Nil(ARTIFACT.module.entries.nils[0].function),
                nils,
            ),
        ];
        // Trace the actual root through its tail transfers and callback returns.
        // The generated completion must preserve the original typed owner.
        let mut artifact = GENERATED_ARTIFACT;
        artifact.module.program.compiled.function_calls = artifact
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .map(|row| {
                let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
                    panic!("the primitive fixture must contain generated calls");
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
                            start: replacements
                                .iter()
                                .find(|(target, _)| *target == row.function)
                                .map_or(body.start, |(_, start)| *start),
                        })
                        .into(),
                    ),
                }
            })
            .collect::<Vec<_>>()
            .into();
        Box::leak(Box::new(artifact))
    }

    pub(super) fn reset() {
        CHARGES.lock().unwrap().clear();
    }

    pub(super) fn check(name: &str, expected_steps: Option<usize>) {
        let charges = CHARGES.lock().unwrap();
        assert!(
            charges.len() > 1,
            "{name} must actually yield and resume generated execution"
        );
        assert_eq!(charges[0].offered, 0, "{name}");
        assert_eq!(charges[0].consumed, 0, "{name}");
        assert!(!charges[0].complete, "{name}");
        for charge in &charges[1..] {
            assert_eq!(charge.offered, 1, "{name}");
            assert!(charge.consumed <= 1, "{name} exceeded its shared allowance");
        }
        assert!(
            charges.iter().all(|charge| !charge.canonical),
            "{name} must remain generated throughout the callback chain"
        );
        assert_eq!(
            charges.iter().filter(|charge| charge.complete).count(),
            1,
            "{name}"
        );
        assert!(charges.last().unwrap().complete, "{name}");
        if let Some(expected) = expected_steps {
            assert_eq!(
                charges.iter().map(|charge| charge.consumed).sum::<usize>(),
                expected,
                "{name}"
            );
        }
    }
}

#[test]
fn primitive_list_artifact_matches_public_preparation_and_selects_every_return_family() {
    assert_eq!(
        declarations::prepare().emit_rust(),
        include_str!("fixtures/prepared/primitive_list_calls.rs").trim(),
    );
    let mut families = [false; 16];
    for entry in ARTIFACT.module.program.compiled.function_calls.iter() {
        assert!(matches!(
            entry.implementation,
            data::compiled::CompiledImplementation::FunctionCalls(_)
        ));
        let family = match entry.function {
            data::compiled::CallTarget::Int(_) => 0,
            data::compiled::CallTarget::Bool(_) => 1,
            data::compiled::CallTarget::IntFunction(_) => 2,
            data::compiled::CallTarget::BoolFunction(_) => 3,
            data::compiled::CallTarget::Float(_) => 4,
            data::compiled::CallTarget::String(_) => 5,
            data::compiled::CallTarget::BitArray(_) => 6,
            data::compiled::CallTarget::UtfCodepoint(_) => 7,
            data::compiled::CallTarget::Nil(_) => 8,
            data::compiled::CallTarget::FloatFunction(_) => 9,
            data::compiled::CallTarget::StringFunction(_) => 10,
            data::compiled::CallTarget::BitArrayFunction(_) => 11,
            data::compiled::CallTarget::UtfCodepointFunction(_) => 12,
            data::compiled::CallTarget::NilFunction(_) => 13,
            data::compiled::CallTarget::Custom(_) => 14,
            data::compiled::CallTarget::Tuple(_) => 15,
        };
        families[family] = true;
    }
    assert_eq!(
        families,
        [
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            false, false
        ]
    );
}

#[cfg(feature = "tokio")]
#[test]
fn an_unconnected_bit_callback_keeps_original_execution_and_source_failure() {
    use data::compiled::{CallTarget, CompiledImplementation};
    use data::graph::FunctionTarget;
    use geam_core::embedding::CallError;
    use geam_core::execution::TokioHost;
    use geam_core::{ExecutionError, PanicKind};

    let export = ARTIFACT
        .module
        .exports
        .iter()
        .find(|export| export.name.as_ref() == "call_unconnected_bit_checksum")
        .unwrap();
    let target = CallTarget::Int(ARTIFACT.module.entries.ints[export.slot].function);
    let caller = ARTIFACT
        .module
        .program
        .compiled
        .function_calls
        .iter()
        .find(|function| function.function == target)
        .unwrap();
    let CompiledImplementation::FunctionCalls(body) = &caller.implementation else {
        panic!("the caller must preserve its generated dynamic calls");
    };
    assert!(body.root);
    assert_eq!(body.creations.len(), 2);
    let FunctionTarget::Int(checksum) = body.creations[0].target else {
        panic!("the captured checksum must return Int");
    };
    let FunctionTarget::Int(constant) = body.creations[1].target else {
        panic!("the complete constant callback must return Int");
    };
    assert!(
        !ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .any(|function| function.function == CallTarget::Int(checksum))
    );
    assert!(
        ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .any(|function| function.function == CallTarget::Int(constant))
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let declaration =
            FunctionDeclaration::<(BitArrayValue,), BigInt>::new("call_unconnected_bit_checksum");
        let (mut module, calculate) = if prepared {
            let mut bindings = ARTIFACT
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let calculate = bindings.function(declaration).unwrap();
            (bindings.seal(), calculate)
        } else {
            let (bindings, calculate) = declarations::builder().function(declaration).unwrap();
            (bindings.seal().unwrap(), calculate)
        };
        let mut echo = Vec::new();
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    for (bytes, expected) in [(vec![], 7), (vec![1, 2, 3], 13), (vec![1; 100], 107)]
                    {
                        assert_eq!(
                            scope
                                .call(&calculate, (BitArrayValue::from_bytes(bytes),))
                                .await
                                .unwrap(),
                            BigInt::from(expected)
                        );
                    }
                    let error = scope
                        .call(
                            &calculate,
                            (BitArrayValue::try_from_parts(vec![0x80], 1).unwrap(),),
                        )
                        .await
                        .unwrap_err();
                    let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                        panic!("an incomplete byte must keep its source panic");
                    };
                    assert_eq!(panic.kind(), PanicKind::Panic);
                    assert_eq!(panic.site().function(), "checksum_bytes");
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        assert!(echo.is_empty());
    }
}

#[cfg(feature = "tokio")]
mod kernel_execution {
    use super::{ARTIFACT, GENERATED_ARTIFACT};
    use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallProgress, CallStorage};
    use data::compiled::{
        CallTarget, CompiledFunction, CompiledImplementation, FunctionCallsImplementation,
    };
    use geam_core::__prepared_support as data;
    use std::sync::Mutex;

    static BOUNDARIES: Mutex<Vec<bool>> = Mutex::new(Vec::new());
    struct Traced(Box<dyn CallExecution>);

    impl CallExecution for Traced {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            // A single instruction tests kernel checkpoints as well as returns.
            let mut remaining = (*budget).min(1);
            let offered = remaining;
            let progress = self.0.advance(ops, &mut remaining);
            *budget -= offered - remaining;
            assert!(
                matches!(
                    &progress,
                    CallProgress::Yield(_) | CallProgress::Complete { .. }
                ),
                "a connected kernel must not re-enter the canonical caller or callee"
            );
            BOUNDARIES
                .lock()
                .unwrap()
                .push(matches!(&progress, CallProgress::Complete { .. }));
            match progress {
                CallProgress::Yield(next) => CallProgress::Yield(Box::new(Self(next))),
                progress => progress,
            }
        }
    }
    fn target(name: &str) -> CallTarget {
        let slot = ARTIFACT
            .module
            .exports
            .iter()
            .find(|entry| entry.name.as_ref() == name)
            .unwrap()
            .slot;
        match name {
            "call_prefix_length" => CallTarget::Int(ARTIFACT.module.entries.ints[slot].function),
            "call_string_slice" => {
                CallTarget::String(ARTIFACT.module.entries.strings[slot].function)
            }
            "call_bit_slice" => {
                CallTarget::BitArray(ARTIFACT.module.entries.bit_arrays[slot].function)
            }
            _ => panic!("kernel fixture target must have an explicit return family"),
        }
    }
    fn start(
        target: CallTarget,
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        let row = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .find(|row| row.function == target)
            .unwrap();
        let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
            panic!("the source wrapper must own generated execution");
        };
        (body.start)(point, inputs, storage)
            .map(|execution| Box::new(Traced(execution)) as Box<dyn CallExecution>)
    }
    fn prefixes(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(target("call_prefix_length"), point, inputs, storage)
    }
    fn strings(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(target("call_string_slice"), point, inputs, storage)
    }
    fn bits(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start(target("call_bit_slice"), point, inputs, storage)
    }
    pub(super) fn reset() {
        BOUNDARIES.lock().unwrap().clear();
    }
    pub(super) fn artifact() -> &'static data::HostedModuleArtifact {
        let mut artifact = GENERATED_ARTIFACT;
        artifact.module.program.compiled.function_calls = artifact
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .map(|row| {
                let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
                    panic!("the fixture table owns call contracts");
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
                            start: if row.function == target("call_prefix_length") {
                                prefixes
                            } else if row.function == target("call_string_slice") {
                                strings
                            } else if row.function == target("call_bit_slice") {
                                bits
                            } else {
                                body.start
                            },
                        })
                        .into(),
                    ),
                }
            })
            .collect::<Vec<_>>()
            .into();
        BOUNDARIES.lock().unwrap().clear();
        Box::leak(Box::new(artifact))
    }
    pub(super) fn check() {
        let boundaries = BOUNDARIES.lock().unwrap();
        assert!(
            boundaries.len() > 3,
            "single-instruction allowance must yield within the kernel"
        );
        assert_eq!(boundaries.iter().filter(|complete| **complete).count(), 1);
        assert_eq!(boundaries.last(), Some(&true));
    }
}

#[cfg(feature = "tokio")]
#[test]
fn generated_callers_keep_string_and_bit_kernels_owned_through_calls_tails_and_yields() {
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut bindings = kernel_execution::artifact()
        .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
        .unwrap();
    let prefixes = bindings
        .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
            "call_prefix_length",
        ))
        .unwrap();
    let strings = bindings
        .function(FunctionDeclaration::<(StringValue, bool), StringValue>::new("call_string_slice"))
        .unwrap();
    let bits = bindings
        .function(
            FunctionDeclaration::<(BitArrayValue, bool), BitArrayValue>::new("call_bit_slice"),
        )
        .unwrap();
    let mut module = bindings.seal();
    runtime
        .block_on(
            module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                assert_eq!(
                    scope
                        .call(&prefixes, (StringValue::from("xxx"),))
                        .await
                        .unwrap(),
                    BigInt::from(3)
                );
                kernel_execution::check();
                for tail in [false, true] {
                    kernel_execution::reset();
                    let text = StringValue::from("tag:tag:λ-owned-slice-beyond-call-frame");
                    let pointer = text.as_bytes().as_ptr();
                    let result = scope.call(&strings, (text, tail)).await.unwrap();
                    assert_eq!(
                        result.as_bytes(),
                        "λ-owned-slice-beyond-call-frame".as_bytes()
                    );
                    assert_eq!(result.as_bytes().as_ptr(), pointer.wrapping_add(8));
                    kernel_execution::check();
                    kernel_execution::reset();
                    let value = BitArrayValue::try_from_parts(vec![0xE5, 0x58], 13).unwrap();
                    let result = scope.call(&bits, (value, tail)).await.unwrap();
                    assert_eq!(
                        result,
                        BitArrayValue::try_from_parts(vec![0x95, 0x60], 11).unwrap()
                    );
                    kernel_execution::check();
                }
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
}

#[cfg(feature = "tokio")]
#[test]
fn a_kernel_result_crosses_a_canonical_effect_once_and_keeps_source_failure_origin() {
    use geam_core::embedding::CallError;
    use geam_core::execution::TokioHost;
    use geam_core::{ExecutionError, PanicKind};
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, echo_slice, prefixes) = if prepared {
            let mut bindings = ARTIFACT
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let echo_slice = bindings
                .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                    "call_string_slice_echo",
                ))
                .unwrap();
            let prefixes = bindings
                .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
                    "call_prefix_length",
                ))
                .unwrap();
            (bindings.seal(), echo_slice, prefixes)
        } else {
            let (mut bindings, echo_slice) = declarations::builder()
                .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                    "call_string_slice_echo",
                ))
                .unwrap();
            let prefixes = bindings
                .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
                    "call_prefix_length",
                ))
                .unwrap();
            (bindings.seal().unwrap(), echo_slice, prefixes)
        };
        let mut echo = Vec::new();
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    assert_eq!(
                        scope
                            .call(&echo_slice, (StringValue::from("tag:tag:owned"),))
                            .await
                            .unwrap(),
                        StringValue::from("owned")
                    );
                    let error = scope
                        .call(&prefixes, (StringValue::from("xx?"),))
                        .await
                        .unwrap_err();
                    let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                        panic!("source prefix failure must remain a source panic");
                    };
                    assert_eq!(panic.kind(), PanicKind::Panic);
                    assert_eq!(panic.site().function(), "prefix_length");
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(echo.len(), 1);
        assert_eq!(echo[0].value().inspect().to_string(), "\"tag:owned\"");
    }
}

#[cfg(feature = "tokio")]
macro_rules! select_forwarded_functions {
    ($bindings:ident, $integers:expr) => {
        (
            $integers,
            $bindings
                .function(
                    FunctionDeclaration::<(List<f64>, f64, List<f64>), f64>::new("through_floats"),
                )
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<bool>, bool, List<bool>), bool>::new(
                        "through_booleans",
                    ),
                )
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<StringValue>, StringValue, List<StringValue>),
                    StringValue,
                >::new("through_strings"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<BitArrayValue>, BitArrayValue, List<BitArrayValue>),
                    BitArrayValue,
                >::new("through_bit_arrays"))
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<char>, char, List<char>), char>::new(
                        "through_codepoints",
                    ),
                )
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<()>, (), List<()>), ()>::new(
                    "through_nils",
                ))
                .unwrap(),
        )
    };
}

#[cfg(feature = "tokio")]
#[test]
fn primitive_function_results_cross_ordinary_and_tail_call_returns() {
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, functions) = if prepared {
            let mut bindings = ARTIFACT
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let integers = bindings
                .function(FunctionDeclaration::<
                    (List<BigInt>, BigInt, List<BigInt>),
                    BigInt,
                >::new("through_integers"))
                .unwrap();
            let functions = select_forwarded_functions!(bindings, integers);
            (bindings.seal(), functions)
        } else {
            let (mut bindings, integers) = declarations::builder()
                .function(FunctionDeclaration::<
                    (List<BigInt>, BigInt, List<BigInt>),
                    BigInt,
                >::new("through_integers"))
                .unwrap();
            let functions = select_forwarded_functions!(bindings, integers);
            (bindings.seal().unwrap(), functions)
        };
        let (integers, floats, booleans, strings, bit_arrays, codepoints, nils) = functions;
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                    for count in [0, 2] {
                        assert_eq!(
                            scope
                                .call(
                                    &integers,
                                    (
                                        vec![BigInt::from(4); count],
                                        BigInt::from(7),
                                        vec![BigInt::from(100)],
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(if count == 0 { 7 } else { 4 })
                        );
                        assert_eq!(
                            scope
                                .call(&floats, (vec![4.5; count], 7.5, vec![100.5]))
                                .await
                                .unwrap(),
                            if count == 0 { 6.5 } else { 3.5 }
                        );
                        assert_eq!(
                            scope
                                .call(&booleans, (vec![false; count], true, vec![false]))
                                .await
                                .unwrap(),
                            count == 0
                        );
                        let initial = StringValue::from("시작");
                        let prefix = StringValue::from("마지막😀");
                        assert_eq!(
                            scope
                                .call(
                                    &strings,
                                    (
                                        vec![prefix.clone(); count],
                                        initial.clone(),
                                        vec![StringValue::from("입력")],
                                    )
                                )
                                .await
                                .unwrap(),
                            if count == 0 {
                                initial
                            } else {
                                StringValue::from("지막😀")
                            }
                        );
                        let initial = BitArrayValue::try_from_parts(vec![0x80], 1).unwrap();
                        let prefix = BitArrayValue::try_from_parts(vec![0xA5, 0x58], 13).unwrap();
                        assert_eq!(
                            scope
                                .call(
                                    &bit_arrays,
                                    (
                                        vec![prefix.clone(); count],
                                        initial.clone(),
                                        vec![BitArrayValue::from_bytes(vec![7])],
                                    )
                                )
                                .await
                                .unwrap(),
                            if count == 0 { initial } else { prefix }
                        );
                        assert_eq!(
                            scope
                                .call(&codepoints, (vec!['😀'; count], '가', vec!['나']))
                                .await
                                .unwrap(),
                            if count == 0 { '가' } else { '😀' }
                        );
                        scope
                            .call(&nils, (vec![(); count], (), vec![()]))
                            .await
                            .unwrap();
                    }
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
    }
}

// Both binding owners expose the same exact declarations. Selection remains
// explicit here; the source and expected values below are the semantic oracle.
#[cfg(feature = "tokio")]
macro_rules! select_functions {
    ($bindings:ident, $integers:expr) => {
        (
            $integers,
            $bindings
                .function(FunctionDeclaration::<(List<f64>, f64, f64, f64), f64>::new(
                    "floats",
                ))
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<bool>, bool, bool, bool), bool>::new("booleans"),
                )
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<StringValue>, StringValue, StringValue, StringValue),
                    StringValue,
                >::new("strings"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (
                        List<BitArrayValue>,
                        BitArrayValue,
                        BitArrayValue,
                        BitArrayValue,
                    ),
                    BitArrayValue,
                >::new("bit_arrays"))
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<char>, char, char, char), char>::new("codepoints"),
                )
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<()>, (), (), ()), ()>::new(
                    "nils",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<BigInt>, BigInt),
                    CallableType<(List<BigInt>,), BigInt>,
                >::new("make_integers"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<f64>, f64),
                    CallableType<(List<f64>,), f64>,
                >::new("make_floats"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<bool>, bool),
                    CallableType<(List<bool>,), bool>,
                >::new("make_booleans"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<StringValue>, StringValue),
                    CallableType<(List<StringValue>,), StringValue>,
                >::new("make_strings"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<BitArrayValue>, BitArrayValue),
                    CallableType<(List<BitArrayValue>,), BitArrayValue>,
                >::new("make_bit_arrays"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<char>, char),
                    CallableType<(List<char>,), char>,
                >::new("make_codepoints"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<()>, ()),
                    CallableType<(List<()>,), ()>,
                >::new("make_nils"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<f64>,), BigInt>::new(
                    "count_floats",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<()>,), BigInt>::new(
                    "count_nils",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(f64, f64), f64>::new("divide"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<f64>, f64), f64>::new(
                    "canonical_floats",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<f64>, f64), f64>::new(
                    "stopped_float",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(List<(BigInt, f64)>,), BigInt>::new(
                    "unsupported",
                ))
                .unwrap(),
        )
    };
}

#[cfg(feature = "tokio")]
#[test]
fn every_primitive_traversal_keeps_one_budget_through_yields_and_nested_callbacks() {
    use geam_core::execution::TokioHost;
    let mut bindings = bounded_execution::artifact()
        .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
        .unwrap();
    let integers = bindings
        .function(FunctionDeclaration::<
            (List<BigInt>, BigInt, BigInt, BigInt),
            BigInt,
        >::new("integers"))
        .unwrap();
    let (integers, floats, booleans, strings, bit_arrays, codepoints, nils, ..) =
        select_functions!(bindings, integers);
    let mut module = bindings.seal();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    runtime
        .block_on(
            module.with_execution(&host, &mut (), &mut drop, async |scope| {
                for count in [0, 1, 3, 100] {
                    bounded_execution::reset();
                    assert_eq!(
                        scope
                            .call(
                                &integers,
                                (vec![BigInt::from(2); count], 7.into(), 3.into(), 4.into())
                            )
                            .await
                            .unwrap(),
                        BigInt::from(7 + 11 * count)
                    );
                    bounded_execution::check("Int", None);
                    bounded_execution::reset();
                    assert_eq!(
                        scope
                            .call(&floats, (vec![2.0; count], 7.0, 3.0, 4.0))
                            .await
                            .unwrap(),
                        7.0 + 10.0 * count as f64
                    );
                    bounded_execution::check("Float", None);
                    bounded_execution::reset();
                    assert_eq!(
                        scope
                            .call(&booleans, (vec![true; count], false, false, false))
                            .await
                            .unwrap(),
                        count % 2 != 0
                    );
                    bounded_execution::check("Bool", None);
                    let initial = StringValue::from("initial");
                    let text = StringValue::from("λ shared text");
                    bounded_execution::reset();
                    assert_eq!(
                        scope
                            .call(
                                &strings,
                                (
                                    vec![text.clone(); count],
                                    initial.clone(),
                                    initial.clone(),
                                    initial.clone()
                                )
                            )
                            .await
                            .unwrap(),
                        if count == 0 { initial } else { text }
                    );
                    bounded_execution::check("String", None);
                    let initial = BitArrayValue::from_bytes(vec![0]);
                    let bits = BitArrayValue::try_from_parts(vec![0xA5, 0x58], 13).unwrap();
                    bounded_execution::reset();
                    assert_eq!(
                        scope
                            .call(
                                &bit_arrays,
                                (
                                    vec![bits.clone(); count],
                                    initial.clone(),
                                    initial.clone(),
                                    initial.clone()
                                )
                            )
                            .await
                            .unwrap(),
                        if count == 0 { initial } else { bits }
                    );
                    bounded_execution::check("BitArray", None);
                    bounded_execution::reset();
                    assert_eq!(
                        scope
                            .call(&codepoints, (vec!['😀'; count], 'λ', 'x', 'y'))
                            .await
                            .unwrap(),
                        if count == 0 { 'λ' } else { '😀' }
                    );
                    bounded_execution::check("UtfCodepoint", None);
                    bounded_execution::reset();
                    scope
                        .call(&nils, (vec![(); count], (), (), ()))
                        .await
                        .unwrap();
                    // Root creates two callables and tail-calls fold, whose entry
                    // costs one step. Empty/Return complete the traversal.
                    // Each element costs Empty/Index/Tail, callback call,
                    // transform call/Return, keep tail/Return, and fold loop
                    // backedge: nine steps. Self recursion is a graph edge,
                    // so only the initial root tail enters a new function.
                    bounded_execution::check("Nil", Some(6 + 9 * count));
                }
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
}

#[cfg(feature = "tokio")]
#[test]
fn primitive_traversals_and_escaped_callables_match_canonical_execution() {
    use geam_core::embedding::CallError;
    use geam_core::execution::TokioHost;
    use geam_core::{ExecutionError, PanicKind, PanicMessage, SourceSpan};
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, functions) = if prepared {
            let mut bindings = ARTIFACT
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let integers = bindings
                .function(FunctionDeclaration::<
                    (List<BigInt>, BigInt, BigInt, BigInt),
                    BigInt,
                >::new("integers"))
                .unwrap();
            let functions = select_functions!(bindings, integers);
            (bindings.seal(), functions)
        } else {
            let (mut bindings, integers) = declarations::builder()
                .function(FunctionDeclaration::<
                    (List<BigInt>, BigInt, BigInt, BigInt),
                    BigInt,
                >::new("integers"))
                .unwrap();
            let functions = select_functions!(bindings, integers);
            (bindings.seal().unwrap(), functions)
        };
        let (
            integers,
            floats,
            booleans,
            strings,
            bit_arrays,
            codepoints,
            nils,
            make_integers,
            make_floats,
            make_booleans,
            make_strings,
            make_bit_arrays,
            make_codepoints,
            make_nils,
            count_floats,
            count_nils,
            divide,
            canonical_floats,
            stopped_float,
            unsupported,
        ) = functions;
        let mut echo = Vec::new();
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    for count in [0, 1, 100, 10_000] {
                        assert_eq!(
                            scope
                                .call(
                                    &integers,
                                    (
                                        vec![BigInt::from(2); count],
                                        BigInt::from(7),
                                        BigInt::from(3),
                                        BigInt::from(4)
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(7 + 11 * count)
                        );
                        assert_eq!(
                            scope
                                .call(&floats, (vec![2.0; count], 7.0, 3.0, 4.0))
                                .await
                                .unwrap(),
                            7.0 + 10.0 * count as f64
                        );
                        assert_eq!(
                            scope
                                .call(&booleans, (vec![true; count], false, false, false))
                                .await
                                .unwrap(),
                            count % 2 != 0
                        );
                        let initial = StringValue::from("initial");
                        let text = StringValue::from("λ-final-value-with-shared-string-storage");
                        assert_eq!(
                            scope
                                .call(
                                    &strings,
                                    (
                                        vec![text.clone(); count],
                                        initial.clone(),
                                        initial.clone(),
                                        initial.clone()
                                    )
                                )
                                .await
                                .unwrap(),
                            if count == 0 { initial } else { text }
                        );
                        let initial = BitArrayValue::from_bytes(vec![0]);
                        let bits = BitArrayValue::try_from_parts(vec![0xA5, 0x58], 13).unwrap();
                        assert_eq!(
                            scope
                                .call(
                                    &bit_arrays,
                                    (
                                        vec![bits.clone(); count],
                                        initial.clone(),
                                        initial.clone(),
                                        initial.clone()
                                    )
                                )
                                .await
                                .unwrap(),
                            if count == 0 { initial } else { bits }
                        );
                        assert_eq!(
                            scope
                                .call(&codepoints, (vec!['😀'; count], 'λ', 'x', 'y'))
                                .await
                                .unwrap(),
                            if count == 0 { 'λ' } else { '😀' }
                        );
                        scope
                            .call(&nils, (vec![(); count], (), (), ()))
                            .await
                            .unwrap();
                        assert_eq!(
                            scope
                                .call(&count_floats, (vec![1.0; count],))
                                .await
                                .unwrap(),
                            BigInt::from(count)
                        );
                        assert_eq!(
                            scope.call(&count_nils, (vec![(); count],)).await.unwrap(),
                            BigInt::from(count)
                        );
                    }
                    let integer = scope
                        .call(
                            &make_integers,
                            (vec![BigInt::from(3), BigInt::from(4)], BigInt::from(7)),
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        scope
                            .invoke(&integer, (vec![BigInt::from(100)],))
                            .await
                            .unwrap(),
                        BigInt::from(4)
                    );
                    let float = scope
                        .call(&make_floats, (vec![3.5, 4.5], 7.0))
                        .await
                        .unwrap();
                    assert_eq!(scope.invoke(&float, (vec![100.0],)).await.unwrap(), 4.5);
                    let boolean = scope
                        .call(&make_booleans, (vec![true, false], true))
                        .await
                        .unwrap();
                    assert!(!scope.invoke(&boolean, (vec![true],)).await.unwrap());
                    let text = StringValue::from("retained string beyond its initial call");
                    let string = scope
                        .call(
                            &make_strings,
                            (vec![text.clone()], StringValue::from("initial")),
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        scope
                            .invoke(&string, (vec![StringValue::from("other")],))
                            .await
                            .unwrap(),
                        text
                    );
                    let bits = BitArrayValue::from_bytes(vec![1, 2, 3]);
                    let bit_array = scope
                        .call(
                            &make_bit_arrays,
                            (vec![bits.clone()], BitArrayValue::from_bytes(vec![0])),
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        scope
                            .invoke(&bit_array, (vec![BitArrayValue::from_bytes(vec![7])],))
                            .await
                            .unwrap(),
                        bits
                    );
                    let codepoint = scope
                        .call(&make_codepoints, (vec!['😀'], 'λ'))
                        .await
                        .unwrap();
                    assert_eq!(scope.invoke(&codepoint, (vec!['x'],)).await.unwrap(), '😀');
                    let nil = scope.call(&make_nils, (vec![(), ()], ())).await.unwrap();
                    scope.invoke(&nil, (vec![(); 100],)).await.unwrap();

                    for divisor in [0.0, -0.0] {
                        assert_eq!(
                            scope.call(&divide, (7.0, divisor)).await.unwrap().to_bits(),
                            0.0_f64.to_bits()
                        );
                    }
                    assert_eq!(
                        scope.call(&divide, (-0.0, 2.0)).await.unwrap().to_bits(),
                        (-0.0_f64).to_bits()
                    );
                    assert!(scope.call(&divide, (f64::NAN, 2.0)).await.unwrap().is_nan());
                    assert_eq!(
                        scope.call(&divide, (f64::INFINITY, 2.0)).await.unwrap(),
                        f64::INFINITY
                    );
                    assert_eq!(
                        scope
                            .call(&canonical_floats, (vec![2.0, 3.0], 7.0))
                            .await
                            .unwrap(),
                        11.5
                    );
                    let error = scope
                        .call(&stopped_float, (vec![2.0, 3.0], 7.0))
                        .await
                        .unwrap_err();
                    let CallError::Execution(ExecutionError::Panic(panic)) =
                        error.into_materialized()
                    else {
                        panic!("the first Float callback must stop at its source panic");
                    };
                    assert_eq!(panic.kind(), PanicKind::Panic);
                    assert_eq!(
                        panic.message(),
                        &PanicMessage::Explicit("float callback stopped".into())
                    );
                    assert_eq!(panic.site().module(), "example");
                    let source = include_str!("fixtures/prepared/primitive_list_calls.gleam");
                    let stop = "panic as \"float callback stopped\"";
                    let start = source.find(stop).unwrap();
                    assert_eq!(
                        panic.site().span(),
                        SourceSpan::new(start, start + stop.len())
                    );
                    assert_eq!(
                        scope
                            .call(
                                &unsupported,
                                (vec![(BigInt::from(3), 1.0), (BigInt::from(4), 2.0)],)
                            )
                            .await
                            .unwrap(),
                        BigInt::from(7)
                    );
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        // Both canonical callbacks execute once before returning, and the
        // failing callback emits once without running its next list element.
        assert_eq!(
            echo.iter()
                .map(|event| event.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["2.0", "3.0", "9.0"]
        );
    }
}

#[cfg(feature = "tokio")]
macro_rules! select_checks {
    ($bindings:ident, $comparisons:expr) => {
        (
            $comparisons,
            $bindings
                .function(FunctionDeclaration::<
                    (List<BigInt>, BigInt, List<BigInt>),
                    BigInt,
                >::new("check_integers"))
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<f64>, f64, List<f64>), BigInt>::new("check_floats"),
                )
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<bool>, bool, List<bool>), BigInt>::new(
                        "check_booleans",
                    ),
                )
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<StringValue>, StringValue, List<StringValue>),
                    BigInt,
                >::new("check_strings"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<
                    (List<BitArrayValue>, BitArrayValue, List<BitArrayValue>),
                    BigInt,
                >::new("check_bit_arrays"))
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<char>, char, List<char>), BigInt>::new(
                        "check_codepoints",
                    ),
                )
                .unwrap(),
            $bindings
                .function(
                    FunctionDeclaration::<(List<()>, (), List<()>), BigInt>::new("check_nils"),
                )
                .unwrap(),
        )
    };
}

#[cfg(feature = "tokio")]
#[test]
fn primitive_comparisons_and_list_construction_match_canonical_values() {
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, functions) = if prepared {
            let mut bindings = ARTIFACT
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let comparisons = bindings
                .function(FunctionDeclaration::<(f64, f64), BigInt>::new(
                    "float_comparisons",
                ))
                .unwrap();
            let functions = select_checks!(bindings, comparisons);
            (bindings.seal(), functions)
        } else {
            let (mut bindings, comparisons) = declarations::builder()
                .function(FunctionDeclaration::<(f64, f64), BigInt>::new(
                    "float_comparisons",
                ))
                .unwrap();
            let functions = select_checks!(bindings, comparisons);
            (bindings.seal().unwrap(), functions)
        };
        let (comparisons, integers, floats, booleans, strings, bit_arrays, codepoints, nils) =
            functions;
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                    for (left, right, expected) in [
                        (1.0, 2.0, 867),
                        (2.0, 1.0, 876),
                        (-0.0, 0.0, 858),
                        (f64::INFINITY, f64::INFINITY, 858),
                        (f64::NAN, 1.0, 160),
                        (f64::NAN, f64::NAN, 160),
                        (1.0, f64::NAN, 864),
                    ] {
                        assert_eq!(
                            scope.call(&comparisons, (left, right)).await.unwrap(),
                            BigInt::from(expected)
                        );
                    }
                    for count in [0, 1, 2] {
                        // The exact prefix plus the original suffix; distinct expected
                        // lists also exercise content equality rather than handle identity.
                        let score = if count == 0 {
                            1
                        } else if count == 1 {
                            7
                        } else {
                            11
                        };
                        assert_eq!(
                            scope
                                .call(
                                    &integers,
                                    (
                                        vec![BigInt::from(2); count],
                                        BigInt::from(3),
                                        std::iter::once(BigInt::from(3))
                                            .chain(std::iter::repeat_n(BigInt::from(2), count))
                                            .collect::<Vec<_>>()
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                        assert_eq!(
                            scope
                                .call(
                                    &floats,
                                    (
                                        vec![2.5; count],
                                        3.5,
                                        std::iter::once(3.5)
                                            .chain(std::iter::repeat_n(2.5, count))
                                            .collect::<Vec<_>>()
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                        assert_eq!(
                            scope
                                .call(
                                    &booleans,
                                    (
                                        vec![true; count],
                                        false,
                                        std::iter::once(false)
                                            .chain(std::iter::repeat_n(true, count))
                                            .collect::<Vec<_>>()
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                        let text = StringValue::from("λ shared text");
                        let prefix = StringValue::from("prefix");
                        assert_eq!(
                            scope
                                .call(
                                    &strings,
                                    (
                                        vec![text.clone(); count],
                                        prefix.clone(),
                                        std::iter::once(prefix)
                                            .chain(std::iter::repeat_n(text, count))
                                            .collect::<Vec<_>>()
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                        let bits = BitArrayValue::try_from_parts(vec![0xA0], 3).unwrap();
                        let prefix = BitArrayValue::try_from_parts(vec![0x80], 1).unwrap();
                        assert_eq!(
                            scope
                                .call(
                                    &bit_arrays,
                                    (
                                        vec![bits.clone(); count],
                                        prefix.clone(),
                                        std::iter::once(prefix)
                                            .chain(std::iter::repeat_n(bits, count))
                                            .collect::<Vec<_>>()
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                        assert_eq!(
                            scope
                                .call(
                                    &codepoints,
                                    (
                                        vec!['😀'; count],
                                        'λ',
                                        std::iter::once('λ')
                                            .chain(std::iter::repeat_n('😀', count))
                                            .collect::<Vec<_>>()
                                    )
                                )
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                        assert_eq!(
                            scope
                                .call(&nils, (vec![(); count], (), vec![(); count + 1]))
                                .await
                                .unwrap(),
                            BigInt::from(score)
                        );
                    }
                    assert_eq!(
                        scope
                            .call(&floats, (vec![1.0], f64::NAN, vec![f64::NAN, 1.0]))
                            .await
                            .unwrap(),
                        BigInt::from(6)
                    );
                    assert_eq!(
                        scope
                            .call(&floats, (vec![1.0], -0.0, vec![0.0, 1.0]))
                            .await
                            .unwrap(),
                        BigInt::from(7)
                    );
                    assert_eq!(
                        scope
                            .call(
                                &strings,
                                (
                                    vec![],
                                    StringValue::from("left"),
                                    vec![StringValue::from("right")]
                                )
                            )
                            .await
                            .unwrap(),
                        BigInt::from(2)
                    );
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
    }
}
