use super::BlockLayout;
use crate::plan::execution::graph::{
    FamilyTransfer, MatchEdgeArgument, ParamLocal, ParamSlot, StorageFamily, StorageSlot, Transfer,
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
            .map(|(family, count)| FamilyTransfer {
                family,
                positions: positions(count, &outputs.remove(&family).unwrap_or_default()).into(),
            })
            .collect(),
    }
}

fn positions(count: usize, outputs: &[usize]) -> Vec<usize> {
    let mut labels = (0..count).collect::<Vec<_>>();
    let mut locations = labels.clone();
    let mut positions = Vec::with_capacity(outputs.len());
    for (destination, &origin) in outputs.iter().enumerate() {
        let source = locations[origin];
        positions.push(source);
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
    positions
}

#[cfg(test)]
mod tests {
    use super::positions;

    #[test]
    fn routes_reordered_and_repeated_outputs_without_losing_later_sources() {
        assert_eq!(positions(3, &[2, 0, 2]), [2, 2, 0]);
        assert_eq!(positions(3, &[0, 0, 1, 2, 1]), [0, 0, 3, 3, 2]);
        assert_eq!(positions(1, &[0, 0, 0]), [0, 0, 0]);
        assert!(positions(3, &[]).is_empty());
        assert!(positions(0, &[]).is_empty());
    }

    #[test]
    fn every_small_argument_pack_moves_first_uses_and_copies_only_repeats() {
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
                    let route = positions(count, &outputs);
                    assert_eq!(route.len(), outputs.len());
                    let mut values = (0..count).collect::<Vec<_>>();
                    let mut copies = 0;
                    for (destination, source) in route.into_iter().enumerate() {
                        if source < destination {
                            values.push(values[source]);
                            let last = values.len() - 1;
                            values.swap(last, destination);
                            copies += 1;
                        } else {
                            values.swap(source, destination);
                        }
                    }
                    values.truncate(outputs.len());
                    assert_eq!(values, outputs);
                    let unique = outputs
                        .iter()
                        .collect::<std::collections::BTreeSet<_>>()
                        .len();
                    assert_eq!(copies, outputs.len() - unique);
                }
            }
        }
    }
}
