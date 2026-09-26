use super::BlockLayout;
use crate::plan::execution::graph::{
    FamilyTransfer, MatchEdgeArgument, ParamLocal, ParamSlot, StorageFamily, StorageSlot, Transfer,
    TransferStep,
};
use std::collections::BTreeMap;

pub(super) fn arguments(source: &BlockLayout, args: &[ParamLocal]) -> Transfer {
    freeze(
        counts(source),
        args.iter().filter_map(ParamLocal::storage_slot),
    )
}

pub(super) fn matched(
    source: &BlockLayout,
    args: &[MatchEdgeArgument],
    params: &[ParamSlot],
) -> (Vec<usize>, Transfer) {
    let mut counts = counts(source);
    let mut bindings = BTreeMap::new();
    for (arg, param) in args.iter().zip(params) {
        if let MatchEdgeArgument::Binding(index) = arg {
            bindings.insert(*index, param.local().storage_slot());
        }
    }
    for slot in bindings.values_mut().flatten() {
        let count = counts.entry(slot.family).or_default();
        slot.index = *count;
        *count += 1;
    }
    let transfer = freeze(
        counts,
        args.iter().filter_map(|arg| match arg {
            MatchEdgeArgument::Binding(index) => bindings[index],
            MatchEdgeArgument::Value(local) => local.storage_slot(),
        }),
    );
    (bindings.into_keys().collect(), transfer)
}

fn counts(source: &BlockLayout) -> BTreeMap<StorageFamily, usize> {
    let mut counts = BTreeMap::<_, usize>::new();
    for slot in source.values.locals().filter_map(ParamLocal::storage_slot) {
        let count = counts.entry(slot.family).or_default();
        *count = (*count).max(slot.index + 1);
    }
    counts
}

fn freeze(
    counts: BTreeMap<StorageFamily, usize>,
    args: impl Iterator<Item = StorageSlot>,
) -> Transfer {
    let mut outputs = BTreeMap::<_, Vec<_>>::new();
    for slot in args {
        outputs.entry(slot.family).or_default().push(slot.index);
    }
    Transfer {
        families: counts
            .into_iter()
            .filter_map(|(family, count)| {
                let outputs = outputs.remove(&family).unwrap_or_default();
                let steps = steps(count, &outputs);
                (count != outputs.len() || !steps.is_empty()).then(|| FamilyTransfer {
                    family,
                    length: outputs.len(),
                    steps: steps.into(),
                })
            })
            .collect(),
    }
}

fn steps(count: usize, outputs: &[usize]) -> Vec<TransferStep> {
    let mut labels = (0..count).collect::<Vec<_>>();
    let mut locations = labels.clone();
    let mut steps = Vec::new();
    for (destination, &origin) in outputs.iter().enumerate() {
        let source = locations[origin];
        if source == destination {
            continue;
        }
        steps.push(TransferStep {
            source,
            destination,
        });
        if source < destination {
            labels.push(origin);
            let last = labels.len() - 1;
            labels.swap(last, destination);
            let displaced = labels[last];
            if locations[displaced] == destination {
                locations[displaced] = last;
            }
        } else {
            labels.swap(source, destination);
            locations[labels[source]] = source;
            locations[origin] = destination;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::{freeze, steps};
    use crate::plan::execution::graph::{StorageFamily, StorageSlot, TransferStep};
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn routes_reordered_and_repeated_outputs_without_losing_later_sources() {
        assert_eq!(
            steps(3, &[2, 0, 2]),
            [
                TransferStep {
                    source: 2,
                    destination: 0
                },
                TransferStep {
                    source: 2,
                    destination: 1
                },
                TransferStep {
                    source: 0,
                    destination: 2
                },
            ]
        );
        assert_eq!(
            steps(3, &[0, 0, 1, 2, 1]),
            [
                TransferStep {
                    source: 0,
                    destination: 1
                },
                TransferStep {
                    source: 3,
                    destination: 2
                },
                TransferStep {
                    source: 2,
                    destination: 4
                },
            ]
        );
        assert_eq!(
            steps(1, &[0, 0, 0]),
            [
                TransferStep {
                    source: 0,
                    destination: 1
                },
                TransferStep {
                    source: 0,
                    destination: 2
                },
            ]
        );
        assert!(steps(3, &[]).is_empty());
        assert!(steps(0, &[]).is_empty());
    }

    #[test]
    fn omits_only_complete_identity_families_and_retains_drop_only_routes() {
        let transfer = freeze(
            BTreeMap::from([
                (StorageFamily::Int, 3),
                (StorageFamily::String, 2),
                (StorageFamily::Bool, 1),
            ]),
            [
                StorageSlot {
                    family: StorageFamily::Int,
                    index: 0,
                },
                StorageSlot {
                    family: StorageFamily::String,
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
            ]
            .into_iter(),
        );
        assert_eq!(
            transfer
                .families
                .iter()
                .map(|route| (route.family, route.length, &*route.steps))
                .collect::<Vec<_>>(),
            [
                (StorageFamily::String, 1, &[][..]),
                (StorageFamily::Bool, 0, &[][..]),
            ]
        );
        assert!(
            freeze(BTreeMap::new(), std::iter::empty())
                .families
                .is_empty()
        );
    }

    #[test]
    fn every_small_argument_pack_preserves_outputs_clones_and_discarded_occurrence_order() {
        let mut checked = 0;
        for count in 0usize..=5 {
            for length in 0..=6 {
                for mut encoded in 0..count.pow(length) {
                    let outputs = (0..length)
                        .map(|_| {
                            let origin = encoded % count;
                            encoded /= count;
                            origin
                        })
                        .collect::<Vec<_>>();
                    // Dense first-use routing is the old contract, independently
                    // located by occurrence rather than the production index map.
                    let mut dense = (0..count).map(|id| (id, id)).collect::<Vec<_>>();
                    let mut cloned = Vec::new();
                    for (destination, &origin) in outputs.iter().enumerate() {
                        let source = dense.iter().position(|(id, _)| *id == origin).unwrap();
                        if source < destination {
                            cloned.push(origin);
                            dense.push((origin, count + cloned.len() - 1));
                            let last = dense.len() - 1;
                            dense.swap(last, destination);
                        } else {
                            dense.swap(source, destination);
                        }
                    }
                    let route = steps(count, &outputs);
                    let mut sparse = (0..count).map(|id| (id, id)).collect::<Vec<_>>();
                    let mut copies = Vec::new();
                    for TransferStep {
                        source,
                        destination,
                    } in route
                    {
                        assert_ne!(source, destination);
                        if source < destination {
                            copies.push(sparse[source].0);
                            sparse.push((sparse[source].0, count + copies.len() - 1));
                            let last = sparse.len() - 1;
                            sparse.swap(last, destination);
                        } else {
                            sparse.swap(source, destination);
                        }
                    }
                    assert_eq!(sparse, dense);
                    assert_eq!(copies, cloned);
                    assert_eq!(
                        sparse[..outputs.len()]
                            .iter()
                            .map(|(id, _)| *id)
                            .collect::<Vec<_>>(),
                        outputs
                    );
                    let unique = outputs.iter().collect::<BTreeSet<_>>().len();
                    assert_eq!(copies.len(), outputs.len() - unique);
                    checked += 1;
                }
            }
        }
        assert_eq!(checked, 26220);
    }
}
