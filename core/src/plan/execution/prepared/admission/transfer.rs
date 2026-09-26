use super::local::Locals;
use crate::plan::execution::graph::{
    MatchEdge, MatchEdgeArgument, ParamLocal, ParamSlot, StorageFamily, StorageSlot, Transfer,
};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TransferError {
    Families,
    Arity {
        family: StorageFamily,
        expected: usize,
        found: usize,
    },
    Steps {
        family: StorageFamily,
    },
    Source {
        family: StorageFamily,
        output: usize,
    },
    Bindings,
}

pub(super) fn arguments(
    transfer: &Transfer,
    args: &[ParamLocal],
    locals: &Locals<'_>,
) -> Result<(), TransferError> {
    check(
        transfer,
        counts(locals),
        args.iter().filter_map(ParamLocal::storage_slot),
    )
}

pub(super) fn matched(
    edge: &MatchEdge,
    params: &[ParamSlot],
    locals: &Locals<'_>,
) -> Result<(), TransferError> {
    let mut counts = counts(locals);
    let mut bindings = BTreeMap::new();
    for (arg, param) in edge.args.iter().zip(params) {
        if let MatchEdgeArgument::Binding(index) = arg {
            let family = param.local().storage_slot().map(|slot| slot.family);
            bindings.insert(*index, family);
        }
    }
    if !bindings.keys().copied().eq(edge.bindings.iter().copied()) {
        return Err(TransferError::Bindings);
    }
    let bindings = bindings
        .into_iter()
        .map(|(index, family)| {
            let slot = family.map(|family| {
                let count = counts.entry(family).or_default();
                let slot = StorageSlot {
                    family,
                    index: *count,
                };
                *count += 1;
                slot
            });
            (index, slot)
        })
        .collect::<BTreeMap<_, _>>();
    check(
        &edge.transfer,
        counts,
        edge.args.iter().filter_map(|arg| match arg {
            MatchEdgeArgument::Binding(index) => bindings[index],
            MatchEdgeArgument::Value(local) => local.storage_slot(),
        }),
    )
}

fn counts(locals: &Locals<'_>) -> BTreeMap<StorageFamily, usize> {
    let mut counts = BTreeMap::<_, usize>::new();
    for slot in locals.storage_slots() {
        let count = counts.entry(slot.family).or_default();
        *count = (*count).max(slot.index + 1);
    }
    counts
}

fn check(
    transfer: &Transfer,
    counts: BTreeMap<StorageFamily, usize>,
    args: impl Iterator<Item = StorageSlot>,
) -> Result<(), TransferError> {
    let mut outputs = BTreeMap::<_, Vec<_>>::new();
    for slot in args {
        outputs.entry(slot.family).or_default().push(slot.index);
    }
    let mut routes = transfer.families.iter().peekable();
    for (family, count) in counts {
        let expected = outputs.remove(&family).unwrap_or_default();
        let route = match routes.peek() {
            Some(route) if route.family == family => routes.next(),
            _ => None,
        };
        let Some(route) = route else {
            if expected.into_iter().eq(0..count) {
                continue;
            }
            return Err(TransferError::Families);
        };
        if route.length != expected.len() {
            return Err(TransferError::Arity {
                family,
                expected: expected.len(),
                found: route.length,
            });
        }
        if route
            .steps
            .iter()
            .any(|step| step.source == step.destination || step.destination >= route.length)
            || route
                .steps
                .windows(2)
                .any(|pair| pair[0].destination >= pair[1].destination)
        {
            return Err(TransferError::Steps { family });
        }
        // Track provenance, not runtime values or a reconstructed execution graph.
        let mut origins = (0..count).collect::<Vec<_>>();
        let mut steps = route.steps.iter().peekable();
        for (output, origin) in expected.into_iter().enumerate() {
            let position = match steps.peek() {
                Some(step) if step.destination == output => {
                    let source = step.source;
                    steps.next();
                    source
                }
                _ => output,
            };
            if origins.get(position) != Some(&origin) {
                return Err(TransferError::Source { family, output });
            }
            if position < output {
                origins.push(origin);
                let last = origins.len() - 1;
                origins.swap(last, output);
            } else {
                origins.swap(position, output);
            }
        }
    }
    if routes.next().is_some() {
        return Err(TransferError::Families);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{TransferError, arguments, check, matched};
    use crate::plan::execution::function::FunctionExit;
    use crate::plan::execution::graph::{
        BlockId, FamilyTransfer, IntLocalId, MatchEdge, MatchEdgeArgument, NilLocalId, ParamLocal,
        ParamSlot, StorageFamily, StorageSlot, Transfer, TransferStep,
    };
    use crate::plan::execution::prepared::admission::{local::Locals, type_::Types};
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId};
    use std::collections::BTreeMap;

    #[test]
    fn admits_real_tail_routing_and_rejects_wrong_same_family_sources_and_unbounded_work() {
        let source = r#"
fn finish(a: Int, b: Int, c: Int) {
  case a { 0 -> b + c _ -> a + b + c }
}
fn swap(a: Int, b: Int) { finish(b, a + 0, b) }
pub fn main() { swap(10, 20) }
"#;
        let module = crate::compile_typed_module("example", "example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let mut checked = 0;
        for function in plan.program.functions.value_returns.int_functions.iter() {
            let body = function.body();
            for block in body.block_graph().blocks() {
                let crate::plan::execution::graph::Terminator::Exit(exit) = block.terminator()
                else {
                    continue;
                };
                let FunctionExit::TailCall { args, transfer, .. } = body.exit(*exit) else {
                    continue;
                };
                if args.len() != 3 {
                    continue;
                }
                let mut locals = Locals::default();
                for slot in block.params().iter().chain(
                    block
                        .instructions()
                        .iter()
                        .map(|instruction| instruction.output()),
                ) {
                    locals.define(slot, &types).unwrap();
                }
                assert_eq!(transfer.families.len(), 1);
                assert_eq!(transfer.families[0].family, StorageFamily::Int);
                assert_eq!(transfer.families[0].length, 3);
                assert_eq!(
                    &*transfer.families[0].steps,
                    &[
                        TransferStep {
                            source: 1,
                            destination: 0
                        },
                        TransferStep {
                            source: 3,
                            destination: 1
                        },
                        TransferStep {
                            source: 0,
                            destination: 2
                        },
                    ]
                );
                assert_eq!(arguments(transfer, args, &locals), Ok(()));
                let mut malformed = transfer.clone();
                let mut route = malformed.families[0].clone();
                let mut steps = route.steps.to_vec();
                steps[1].destination = steps[0].destination;
                route.steps = steps.into();
                malformed.families = vec![route].into();
                assert_eq!(
                    arguments(&malformed, args, &locals),
                    Err(TransferError::Steps {
                        family: StorageFamily::Int
                    })
                );
                for (source, output) in [(2, 0), (usize::MAX, 2)] {
                    let mut malformed = transfer.clone();
                    let mut steps = malformed.families[0].steps.to_vec();
                    steps[output].source = source;
                    let mut route = malformed.families[0].clone();
                    route.steps = steps.into();
                    malformed.families = vec![route].into();
                    assert_eq!(
                        arguments(&malformed, args, &locals),
                        Err(TransferError::Source {
                            family: StorageFamily::Int,
                            output,
                        })
                    );
                }
                for length in [2, 4] {
                    let mut route = transfer.families[0].clone();
                    route.length = length;
                    assert_eq!(
                        arguments(
                            &Transfer {
                                families: vec![route].into()
                            },
                            args,
                            &locals
                        ),
                        Err(TransferError::Arity {
                            family: StorageFamily::Int,
                            expected: 3,
                            found: length,
                        })
                    );
                }
                for families in [
                    vec![],
                    vec![FamilyTransfer {
                        family: StorageFamily::String,
                        length: 3,
                        steps: transfer.families[0].steps.clone(),
                    }],
                    vec![transfer.families[0].clone(), transfer.families[0].clone()],
                ] {
                    assert_eq!(
                        arguments(
                            &Transfer {
                                families: families.into()
                            },
                            args,
                            &locals
                        ),
                        Err(TransferError::Families)
                    );
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 1);
    }

    #[test]
    fn validates_sparse_positions_and_omitted_or_truncated_families() {
        let counts = BTreeMap::from([(StorageFamily::Int, 3), (StorageFamily::Bool, 1)]);
        let args = [
            StorageSlot {
                family: StorageFamily::Int,
                index: 0,
            },
            StorageSlot {
                family: StorageFamily::Int,
                index: 1,
            },
            StorageSlot {
                family: StorageFamily::Int,
                index: 2,
            },
        ];
        let discard = FamilyTransfer {
            family: StorageFamily::Bool,
            length: 0,
            steps: Table::Static(&[]),
        };
        assert_eq!(
            check(
                &Transfer {
                    families: vec![discard.clone()].into()
                },
                counts.clone(),
                args.iter().copied()
            ),
            Ok(())
        );
        // A bounded explicit identity family is legal external data too.
        let identity = FamilyTransfer {
            family: StorageFamily::Int,
            length: 3,
            steps: Table::Static(&[]),
        };
        assert_eq!(
            check(
                &Transfer {
                    families: vec![identity.clone(), discard.clone()].into()
                },
                counts.clone(),
                args.iter().copied()
            ),
            Ok(())
        );
        for families in [
            vec![],
            vec![discard.clone(), identity.clone()],
            vec![identity.clone(), discard.clone(), discard.clone()],
        ] {
            assert_eq!(
                check(
                    &Transfer {
                        families: families.into()
                    },
                    counts.clone(),
                    args.iter().copied()
                ),
                Err(TransferError::Families)
            );
        }
        let shrinking = FamilyTransfer {
            family: StorageFamily::Int,
            length: 1,
            steps: Table::Static(&[]),
        };
        assert_eq!(
            check(
                &Transfer {
                    families: vec![shrinking, discard.clone()].into()
                },
                counts.clone(),
                args[..1].iter().copied()
            ),
            Ok(())
        );
        for steps in [
            vec![TransferStep {
                source: 0,
                destination: 0,
            }],
            vec![TransferStep {
                source: 0,
                destination: 3,
            }],
            vec![TransferStep {
                source: 0,
                destination: usize::MAX,
            }],
            vec![
                TransferStep {
                    source: 2,
                    destination: 0,
                },
                TransferStep {
                    source: 1,
                    destination: 0,
                },
            ],
            vec![
                TransferStep {
                    source: 0,
                    destination: 2,
                },
                TransferStep {
                    source: 0,
                    destination: 1,
                },
            ],
        ] {
            let route = FamilyTransfer {
                steps: steps.into(),
                ..identity.clone()
            };
            assert_eq!(
                check(
                    &Transfer {
                        families: vec![route, discard.clone()].into()
                    },
                    counts.clone(),
                    args.iter().copied()
                ),
                Err(TransferError::Steps {
                    family: StorageFamily::Int
                })
            );
        }
        // Omitting a required movement must still fail source provenance.
        let swapped = [args[2], args[1], args[0]];
        assert_eq!(
            check(
                &Transfer {
                    families: vec![identity, discard].into()
                },
                counts,
                swapped.iter().copied()
            ),
            Err(TransferError::Source {
                family: StorageFamily::Int,
                output: 0
            })
        );
        assert_eq!(
            check(
                &Transfer {
                    families: Table::Static(&[])
                },
                BTreeMap::new(),
                [].iter().copied()
            ),
            Ok(())
        );
    }

    #[test]
    fn admits_unique_owned_binding_insertion_nil_and_repeated_binding_outputs() {
        let module =
            crate::compile_typed_module("example", "example.gleam", "pub fn main() { #(42, Nil) }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let shapes = &plan.program.common.value_shapes.shapes;
        let int = ValueShapeId(
            shapes
                .iter()
                .position(|shape| matches!(shape, ValueShapeDescriptor::Int))
                .unwrap(),
        );
        let nil = ValueShapeId(
            shapes
                .iter()
                .position(|shape| matches!(shape, ValueShapeDescriptor::Nil))
                .unwrap(),
        );
        let params = [
            ParamSlot {
                local: ParamLocal::Int(IntLocalId(0)),
                shape: int,
            },
            ParamSlot {
                local: ParamLocal::Int(IntLocalId(1)),
                shape: int,
            },
            ParamSlot {
                local: ParamLocal::Nil(NilLocalId(0)),
                shape: nil,
            },
            ParamSlot {
                local: ParamLocal::Int(IntLocalId(2)),
                shape: int,
            },
        ];
        let mut edge = MatchEdge {
            target: BlockId(0),
            args: vec![
                MatchEdgeArgument::Binding(2),
                MatchEdgeArgument::Binding(0),
                MatchEdgeArgument::Binding(1),
                MatchEdgeArgument::Binding(2),
            ]
            .into(),
            bindings: Table::Static(&[0, 1, 2]),
            transfer: Transfer {
                families: vec![FamilyTransfer {
                    family: StorageFamily::Int,
                    length: 3,
                    steps: Table::Static(&[
                        TransferStep {
                            source: 1,
                            destination: 0,
                        },
                        TransferStep {
                            source: 0,
                            destination: 2,
                        },
                    ]),
                }]
                .into(),
            },
        };
        assert_eq!(matched(&edge, &params, &Locals::default()), Ok(()));
        for bindings in [
            vec![0, 2],
            vec![0, 1, 1, 2],
            vec![2, 1, 0],
            vec![0, 1, 2, 3],
        ] {
            edge.bindings = bindings.into();
            assert_eq!(
                matched(&edge, &params, &Locals::default()),
                Err(TransferError::Bindings)
            );
        }
    }
}
