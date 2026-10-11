use geam_core::__prepared_support as data;
use geam_core::embedding::{BigInt, FunctionDeclaration, List};
#[cfg(feature = "tokio")]
use geam_core::embedding::{CallableType, HostedModuleBuilder};
#[cfg(feature = "tokio")]
use geam_core::{HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile};

static LIST_CALLS: data::HostedModuleArtifact =
    include!("fixtures/prepared/generated/int_list_calls.rs");

#[test]
fn length_only_static_calls_compile_and_execute_without_other_list_operations() {
    static ARTIFACT: data::ModuleArtifact<std::convert::Infallible> =
        include!("fixtures/prepared/generated/int_list_static_calls.rs");
    assert!(ARTIFACT.program.compiled.function_calls.iter().any(|row| {
        row.function == data::compiled::CallTarget::Bool(ARTIFACT.entries.bools[0].function)
    }));
    let mut bindings = ARTIFACT.load().unwrap();
    let nonempty = bindings
        .function(FunctionDeclaration::<(List<BigInt>,), bool>::new(
            "nonempty",
        ))
        .unwrap();
    let module = bindings.seal();
    let mut echo = Vec::new();
    for (values, expected) in [(vec![], false), (vec![7.into()], true)] {
        assert_eq!(
            module.call(&nonempty, (values,), &mut echo).unwrap(),
            expected
        );
    }
    assert!(echo.is_empty());
}

#[cfg(feature = "tokio")]
mod budget_trace {
    use super::LIST_CALLS;
    use data::compiled::calls::{CallExecution, CallInputs, CallOps, CallProgress, CallStorage};
    use data::compiled::{CallTarget, CompiledCheckpoint, CompiledImplementation};
    use geam_core::__prepared_support as data;
    use std::sync::Mutex;

    pub(super) struct Boundary {
        pub point: CompiledCheckpoint,
        pub list_lengths: Vec<usize>,
        pub ints: Vec<Option<i128>>,
    }

    pub(super) struct Charge {
        pub offered: usize,
        pub consumed: usize,
        pub complete: bool,
        pub boundary: Option<Boundary>,
    }

    pub(super) struct Trace {
        pub allowance: usize,
        pub offered_zero: bool,
        pub charges: Vec<Charge>,
    }

    pub(super) static TRACE: Mutex<Trace> = Mutex::new(Trace {
        allowance: 1,
        offered_zero: false,
        charges: Vec::new(),
    });

    struct LimitedLists(Box<dyn CallExecution>);

    impl CallExecution for LimitedLists {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }

        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }

        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            let offered = {
                let mut trace = TRACE.lock().unwrap();
                if trace.offered_zero {
                    trace.allowance.min(*budget)
                } else {
                    trace.offered_zero = true;
                    0
                }
            };
            let mut remaining = offered;
            let progress = self.0.advance(ops, &mut remaining);
            let consumed = offered - remaining;
            *budget -= consumed;
            let boundary = match &progress {
                CallProgress::Interpreted { point, values, .. } => Some(Boundary {
                    point: *point,
                    list_lengths: values.int_lists.iter().map(|list| list.len()).collect(),
                    ints: values.ints.iter().map(|value| value.small()).collect(),
                }),
                _ => None,
            };
            TRACE.lock().unwrap().charges.push(Charge {
                offered,
                consumed,
                complete: matches!(&progress, CallProgress::Complete { .. }),
                boundary,
            });
            match progress {
                CallProgress::Yield(next) => CallProgress::Yield(Box::new(Self(next))),
                progress => progress,
            }
        }
    }

    pub(super) fn wrapper_target() -> CallTarget {
        CallTarget::Int(LIST_CALLS.module.entries.ints[0].function)
    }

    pub(super) fn start(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        let row = LIST_CALLS
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .find(|row| row.function == wrapper_target())
            .unwrap();
        let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
            panic!("the fold must have its generated entry");
        };
        (body.start)(point, inputs, storage)
            .map(|execution| Box::new(LimitedLists(execution)) as Box<dyn CallExecution>)
    }
}

#[cfg(feature = "tokio")]
macro_rules! select_list_calls {
    ($bindings:ident, $fold:expr) => {{
        let fold = $fold;
        let verify = $bindings
            .function(FunctionDeclaration::<(List<BigInt>, BigInt), bool>::new(
                "verify",
            ))
            .unwrap();
        let make_sum = $bindings
            .function(FunctionDeclaration::<
                (List<BigInt>, BigInt),
                CallableType<(List<BigInt>,), BigInt>,
            >::new("make_sum"))
            .unwrap();
        let make_check = $bindings
            .function(FunctionDeclaration::<
                (List<BigInt>,),
                CallableType<(List<BigInt>,), bool>,
            >::new("make_check"))
            .unwrap();
        let selected = $bindings
            .function(FunctionDeclaration::<
                (List<BigInt>, List<BigInt>, bool),
                BigInt,
            >::new("selected"))
            .unwrap();
        let canonical = $bindings
            .function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new(
                "canonical",
            ))
            .unwrap();
        let failure = $bindings
            .function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new(
                "failure",
            ))
            .unwrap();
        let list_return = $bindings
            .function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new(
                "list_return",
            ))
            .unwrap();
        let non_tail = $bindings
            .function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new(
                "non_tail",
            ))
            .unwrap();
        let main = $bindings
            .function(FunctionDeclaration::<(), BigInt>::new("main"))
            .unwrap();
        (
            fold,
            verify,
            make_sum,
            make_check,
            selected,
            canonical,
            failure,
            list_return,
            non_tail,
            main,
        )
    }};
}

#[cfg(feature = "tokio")]
fn dynamic_list_calls() -> geam_core::HostedTypedProgram<StatelessHostProfile> {
    geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/int_list_calls.gleam",
                include_str!("fixtures/prepared/int_list_calls.gleam"),
            )],
        )],
        HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
    )
    .unwrap()
}

#[test]
fn compiled_list_calls_select_the_nested_call_contract() {
    use data::compiled::{CallContractTarget, CallTarget, CompiledImplementation};
    let root = CallTarget::Int(LIST_CALLS.module.entries.ints[0].function);
    let row = LIST_CALLS
        .module
        .program
        .compiled
        .function_calls
        .iter()
        .find(|row| row.function == root)
        .unwrap();
    let CompiledImplementation::FunctionCalls(root) = &row.implementation else {
        panic!("capturing_fold must have its actual generated calls entry");
    };
    assert!(root.root);
    assert!(root.checkpoints.iter().any(|point| point.int_lists == 1));
    // The exported wrapper may hand off its tail once. The list-owning fold
    // and both layers of calculating callbacks must have generated entries.
    let fold = root.tails[0].target;
    let row = LIST_CALLS
        .module
        .program
        .compiled
        .function_calls
        .iter()
        .find(|row| row.function == fold)
        .unwrap();
    let CompiledImplementation::FunctionCalls(fold) = &row.implementation else {
        panic!("the list traversal must be connected to the calls engine");
    };
    assert!(fold.root);
    assert!(fold.checkpoints.iter().any(|point| point.int_lists == 2));
    assert!(matches!(
        fold.calls[0].target,
        CallContractTarget::IntValue(_)
    ));
    let callbacks = LIST_CALLS
        .module
        .program
        .compiled
        .function_calls
        .iter()
        .filter(|row| {
            matches!(&row.implementation, CompiledImplementation::FunctionCalls(body)
            if body.calls.iter().any(|call| matches!(call.target, CallContractTarget::IntValue(_))))
        })
        .count();
    assert!(
        callbacks >= 2,
        "fold and its nested accumulation callback must both dispatch"
    );
}

#[cfg(feature = "tokio")]
#[test]
fn actual_list_calls_charge_every_boundary_and_fall_back_before_a_big_head() {
    use budget_trace::{TRACE, Trace, wrapper_target};
    use data::compiled::{CompiledFunction, CompiledImplementation, FunctionCallsImplementation};
    use geam_core::execution::TokioHost;
    const BASE: data::HostedModuleArtifact =
        include!("fixtures/prepared/generated/int_list_calls.rs");
    let mut artifact = BASE;
    let target = wrapper_target();
    artifact.module.program.compiled.function_calls = artifact
        .module
        .program
        .compiled
        .function_calls
        .iter()
        .map(|row| {
            let CompiledImplementation::FunctionCalls(body) = &row.implementation else {
                panic!("expected a calls implementation");
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
                            budget_trace::start
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
    let artifact = Box::leak(Box::new(artifact));
    let mut bindings = artifact
        .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
        .unwrap();
    let fold = bindings
        .function(
            FunctionDeclaration::<(List<BigInt>, BigInt, BigInt), BigInt>::new("capturing_fold"),
        )
        .unwrap();
    let mut module = bindings.seal();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    runtime
        .block_on(
            module.with_execution(&host, &mut (), &mut drop, async |scope| {
                // Each item charges EmptyTest, Index, Tail, the accumulation call,
                // the transform call, its region and Return, Add, Return and backedge.
                // The final empty test and fold Return add two independent steps.
                // The now-connected wrapper adds two closures, Constant(0), Tail
                // and the callee entry: five steps paid once per invocation.
                for count in [0, 1, 3, 100] {
                    for allowance in (1..=12).chain([31, 1024]) {
                        *TRACE.lock().unwrap() = Trace {
                            allowance,
                            offered_zero: false,
                            charges: Vec::new(),
                        };
                        assert_eq!(
                            scope
                                .call(&fold, (vec![BigInt::from(2); count], 3.into(), 1.into()))
                                .await
                                .unwrap(),
                            BigInt::from(7 * count)
                        );
                        let trace = TRACE.lock().unwrap();
                        let expected_steps = 7 + 10 * count;
                        let mut completed = 0;
                        assert_eq!(trace.charges[0].offered, 0);
                        for charge in &trace.charges {
                            let expected = charge.offered.min(expected_steps - completed);
                            assert_eq!(
                                charge.consumed, expected,
                                "count {count}, allowance {allowance}, prefix {completed}"
                            );
                            completed += expected;
                            assert_eq!(charge.complete, completed == expected_steps);
                            assert!(charge.boundary.is_none());
                        }
                        assert_eq!(completed, expected_steps);
                        assert!(trace.charges.last().unwrap().complete);
                    }
                }
                *TRACE.lock().unwrap() = Trace {
                    allowance: 64,
                    offered_zero: true,
                    charges: Vec::new(),
                };
                let big: BigInt = BigInt::from(1) << 100;
                assert_eq!(
                    scope
                        .call(&fold, (vec![big.clone(), 2.into()], 2.into(), 3.into()))
                        .await
                        .unwrap(),
                    big * 2 + 10
                );
                let trace = TRACE.lock().unwrap();
                let first = &trace.charges[0];
                assert_eq!(
                    first.consumed, 6,
                    "the wrapper and empty test precede the failed Index preflight"
                );
                let boundary = first.boundary.as_ref().unwrap();
                assert_eq!(boundary.point.block, data::graph::BlockId(2));
                assert_eq!(boundary.point.instruction, 0);
                assert_eq!(boundary.point.int_lists, 1);
                assert_eq!(boundary.point.ints, 1);
                assert_eq!(boundary.list_lengths, [2]);
                assert_eq!(boundary.ints, [Some(0)]);
            }),
        )
        .unwrap()
        .try_into_value()
        .unwrap();
}

#[cfg(feature = "tokio")]
#[test]
fn list_calls_preserve_nested_calculation_captures_edges_and_canonical_effects() {
    use geam_core::embedding::CallError;
    use geam_core::execution::TokioHost;
    use geam_core::{ExecutionError, PanicKind, PanicMessage};
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (
            mut module,
            (
                fold,
                verify,
                make_sum,
                make_check,
                selected,
                canonical,
                failure,
                list_return,
                non_tail,
                main,
            ),
        ) = if prepared {
            let mut bindings = LIST_CALLS
                .load(HostProviderSet::<StatelessHostProfile>::new([]).unwrap())
                .unwrap();
            let fold = bindings
                .function(
                    FunctionDeclaration::<(List<BigInt>, BigInt, BigInt), BigInt>::new(
                        "capturing_fold",
                    ),
                )
                .unwrap();
            let functions = select_list_calls!(bindings, fold);
            (bindings.seal(), functions)
        } else {
            let (mut bindings, fold) = HostedModuleBuilder::new(dynamic_list_calls())
                .unwrap()
                .function(
                    FunctionDeclaration::<(List<BigInt>, BigInt, BigInt), BigInt>::new(
                        "capturing_fold",
                    ),
                )
                .unwrap();
            let functions = select_list_calls!(bindings, fold);
            (bindings.seal().unwrap(), functions)
        };
        let mut echo = Vec::new();
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    for (values, expected) in [(vec![], 0), (vec![4], 11), (vec![1, 2, 3], 21)] {
                        let values: Vec<BigInt> = values.into_iter().map(BigInt::from).collect();
                        assert_eq!(
                            scope
                                .call(&fold, (values, 2.into(), 3.into()))
                                .await
                                .unwrap(),
                            BigInt::from(expected)
                        );
                    }
                    let big: BigInt = BigInt::from(1) << 100;
                    // A Big head falls back before Index; Small multiplication overflows
                    // after its completed calculation. Both retain the original list.
                    for value in [BigInt::from(i64::MAX), big] {
                        assert_eq!(
                            scope
                                .call(&fold, (vec![value.clone(), 2.into()], 2.into(), 3.into()))
                                .await
                                .unwrap(),
                            value * 2 + 10
                        );
                    }
                    // Rejecting a Big scalar entry must leave the original list
                    // available for canonical execution, even after reading its lease.
                    let factor: BigInt = BigInt::from(1) << 100;
                    assert_eq!(
                        scope
                            .call(&fold, (vec![2.into()], factor.clone(), 3.into()))
                            .await
                            .unwrap(),
                        factor * 2 + 3
                    );
                    assert!(
                        scope
                            .call(&verify, (vec![2.into(), 3.into()], 5.into()))
                            .await
                            .unwrap()
                    );
                    assert!(
                        !scope
                            .call(&verify, (vec![2.into(), 3.into()], 6.into()))
                            .await
                            .unwrap()
                    );
                    assert_eq!(
                        scope
                            .call(&selected, (vec![4.into()], vec![5.into()], true))
                            .await
                            .unwrap(),
                        BigInt::from(19)
                    );
                    assert_eq!(
                        scope
                            .call(&selected, (vec![4.into()], vec![5.into()], false))
                            .await
                            .unwrap(),
                        BigInt::from(11)
                    );

                    let calculate = scope
                        .call(&make_sum, (vec![2.into(), 3.into()], 7.into()))
                        .await
                        .unwrap();
                    let alias = calculate.clone();
                    drop(calculate);
                    let other = scope
                        .call(&make_sum, (vec![20.into()], 1.into()))
                        .await
                        .unwrap();
                    assert_eq!(
                        scope
                            .invoke(&alias, (vec![4.into(), 5.into()],))
                            .await
                            .unwrap(),
                        BigInt::from(21)
                    );
                    assert_eq!(
                        scope
                            .invoke(&other, (vec![4.into(), 5.into()],))
                            .await
                            .unwrap(),
                        BigInt::from(30)
                    );
                    assert_eq!(
                        scope.invoke(&alias, (Vec::<BigInt>::new(),)).await.unwrap(),
                        BigInt::from(12)
                    );
                    let check = scope
                        .call(&make_check, (vec![2.into(), 3.into()],))
                        .await
                        .unwrap();
                    assert!(
                        scope
                            .invoke(&check, (vec![2.into(), 3.into()],))
                            .await
                            .unwrap()
                    );
                    assert!(
                        !scope
                            .invoke(&check, (vec![2.into(), 4.into()],))
                            .await
                            .unwrap()
                    );
                    assert!(!scope.invoke(&check, (Vec::<BigInt>::new(),)).await.unwrap());
                    assert_eq!(
                        scope
                            .call(&list_return, (vec![2.into(), 3.into()],))
                            .await
                            .unwrap(),
                        BigInt::from(10)
                    );
                    assert_eq!(
                        scope
                            .call(&non_tail, (vec![BigInt::from(1); 2000],))
                            .await
                            .unwrap(),
                        BigInt::from(2000)
                    );
                    assert_eq!(scope.call(&main, ()).await.unwrap(), BigInt::from(15));
                    assert_eq!(
                        scope
                            .call(&canonical, (vec![2.into(), 3.into()],))
                            .await
                            .unwrap(),
                        BigInt::from(7)
                    );
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(
            echo.iter()
                .map(|event| event.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["2", "3"]
        );
        echo.clear();
        runtime
            .block_on(
                module.with_execution(&host, &mut (), &mut echo, async |scope| {
                    let error = scope
                        .call(&failure, (vec![2.into(), 3.into()],))
                        .await
                        .unwrap_err();
                    let CallError::Execution(ExecutionError::Panic(panic)) =
                        error.into_materialized()
                    else {
                        panic!("expected the first list callback to stop");
                    };
                    assert_eq!(panic.kind(), PanicKind::Panic);
                    assert_eq!(
                        panic.message(),
                        &PanicMessage::Explicit("list callback stopped".into())
                    );
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        assert_eq!(
            echo.iter()
                .map(|event| event.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["2"]
        );
    }
}
