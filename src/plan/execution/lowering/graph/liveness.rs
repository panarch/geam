use super::{DraftBlock, DraftBlockId, DraftGraph, DraftValueKey, DraftValueRef};
use std::collections::HashSet;

pub(super) struct GraphLiveness {
    inherited: Vec<Vec<DraftValueRef>>,
    explicit_params: Vec<Vec<usize>>,
}

impl GraphLiveness {
    pub(super) fn analyze(graph: &DraftGraph) -> Self {
        let mut direct = vec![Vec::new(); graph.next_block];
        let mut definitions = vec![HashSet::new(); graph.next_block];

        for index in 0..graph.next_block {
            let block = &graph.blocks[&DraftBlockId(index)];
            collect_block(block, &mut direct[index], &mut definitions[index]);
        }

        let mut inherited = direct;
        loop {
            let mut changed = false;
            for index in (0..graph.next_block).rev() {
                let block = &graph.blocks[&DraftBlockId(index)];
                let mut next = inherited[index].clone();
                for successor in block.terminator.successors() {
                    for value in &inherited[successor.0] {
                        if !definitions[index].contains(&value.key)
                            && !next.iter().any(|existing| existing.key == value.key)
                        {
                            next.push(value.clone());
                        }
                    }
                }
                next.sort_by_key(|value| value.key);
                if next != inherited[index] {
                    inherited[index] = next;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        let explicit_params = (0..graph.next_block)
            .map(|index| {
                let block_id = DraftBlockId(index);
                let block = &graph.blocks[&block_id];
                if block_id == graph.entry {
                    return (0..block.explicit_params.len()).collect();
                }

                let mut uses = Vec::new();
                for instruction in &block.instructions {
                    instruction.uses(&mut uses);
                }
                block.terminator.uses(&mut uses);
                for successor in block.terminator.successors() {
                    uses.extend(inherited[successor.0].iter().cloned());
                }
                let uses = uses
                    .into_iter()
                    .map(|value| value.key)
                    .collect::<HashSet<_>>();

                block
                    .explicit_params
                    .iter()
                    .enumerate()
                    .filter_map(|(index, value)| uses.contains(&value.key).then_some(index))
                    .collect()
            })
            .collect();

        Self {
            inherited,
            explicit_params,
        }
    }

    pub(super) fn inherited(&self, block: DraftBlockId) -> &[DraftValueRef] {
        &self.inherited[block.0]
    }

    pub(super) fn explicit_params(&self, block: DraftBlockId) -> &[usize] {
        &self.explicit_params[block.0]
    }
}

fn collect_block(
    block: &DraftBlock,
    direct: &mut Vec<DraftValueRef>,
    definitions: &mut HashSet<DraftValueKey>,
) {
    for parameter in &block.explicit_params {
        definitions.insert(parameter.key);
    }
    for instruction in &block.instructions {
        let mut uses = Vec::new();
        instruction.uses(&mut uses);
        collect_uses(uses, direct, definitions);
        definitions.insert(instruction.output().key);
    }
    let mut uses = Vec::new();
    block.terminator.uses(&mut uses);
    collect_uses(uses, direct, definitions);
    direct.sort_by_key(|value| value.key);
}

fn collect_uses(
    uses: Vec<DraftValueRef>,
    direct: &mut Vec<DraftValueRef>,
    definitions: &HashSet<DraftValueKey>,
) {
    for value in uses {
        if !definitions.contains(&value.key)
            && !direct.iter().any(|existing| existing.key == value.key)
        {
            direct.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GraphLiveness;
    use crate::plan::execution::lowering::graph::instruction::DraftIntInstruction;
    use crate::plan::execution::lowering::graph::{DraftGraphBuilder, DraftInt};
    use crate::plan::execution::lowering::specialization::StoredValueShape;

    #[test]
    fn fixed_point_threads_only_values_used_by_transitive_successors() {
        let (mut graph, mut entry) = DraftGraphBuilder::<DraftInt, ()>::new(Vec::new(), Vec::new());
        let live = graph.int_instruction(&mut entry, DraftIntInstruction::Value(1.into()));
        let dead = graph.int_instruction(&mut entry, DraftIntInstruction::Value(2.into()));

        let middle = graph.empty_block(entry.scope().clone());
        let middle_id = middle.id();
        let leaf = graph.empty_block(entry.scope().clone());
        let leaf_id = leaf.id();

        graph.finish_return(leaf, live.clone());
        graph.finish_jump(middle, leaf_id, Vec::new());
        graph.finish_jump(entry, middle_id, Vec::new());

        let liveness = GraphLiveness::analyze(&graph);
        assert_eq!(liveness.inherited(graph.entry), &[]);
        assert_eq!(liveness.inherited(middle_id), &[live.erase()]);
        assert_eq!(liveness.inherited(leaf_id), &[live.erase()]);
        assert!(!liveness.inherited(middle_id).contains(&dead.erase()));
        assert!(!liveness.inherited(leaf_id).contains(&dead.erase()));
    }

    #[test]
    fn non_entry_blocks_keep_only_used_explicit_parameters() {
        let (mut graph, mut entry) = DraftGraphBuilder::<DraftInt, ()>::new(
            vec![(
                crate::plan::execution::lowering::local::LocalKey::new(
                    crate::plan::execution::lowering::local::LocalKind::Int,
                    0,
                ),
                StoredValueShape::Int,
            )],
            Vec::new(),
        );
        let first_source = graph.int_instruction(&mut entry, DraftIntInstruction::Value(1.into()));
        let second_source = graph.int_instruction(&mut entry, DraftIntInstruction::Value(2.into()));
        let first = graph.value_ref(StoredValueShape::Int);
        let second = graph.value_ref(StoredValueShape::Int);
        let target = graph.block(entry.scope().clone(), vec![first.clone(), second.clone()]);
        let target_id = target.id();

        graph.finish_return(target, DraftInt::from_owned(first));
        graph.finish_jump(
            entry,
            target_id,
            vec![first_source.erase(), second_source.erase()],
        );

        let liveness = GraphLiveness::analyze(&graph);
        assert_eq!(liveness.explicit_params(graph.entry), &[0]);
        assert_eq!(liveness.explicit_params(target_id), &[0]);
    }
}
