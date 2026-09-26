use super::super::draft::{
    DraftBlock, DraftBlockId, DraftEdge, DraftMatchEdge, DraftTerminator, DraftValueKey,
    DraftValueRef,
};
use super::{
    BlockLayout, ParameterSource, block_layout, edge_arguments, match_arguments, transfer,
};
use crate::plan::execution::graph::{MatchEdgeArgument, StorageFamily, TransferStep};
use crate::plan::execution::lowering::LoweringContext;
use std::collections::{BTreeMap, HashMap, HashSet};

struct Transitions<'draft> {
    edges: Vec<Transition<'draft>>,
    affected: HashMap<DraftBlockId, Vec<usize>>,
    match_targets: HashSet<DraftBlockId>,
}

struct Transition<'draft> {
    source: DraftBlockId,
    block: &'draft DraftBlock,
    inputs: Inputs<'draft>,
}

#[derive(Clone, Copy)]
enum Inputs<'draft> {
    Edge(&'draft DraftEdge),
    Match(&'draft DraftMatchEdge),
    Call(&'draft [DraftValueRef]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Origin {
    Local(DraftValueKey),
    Binding(usize),
}

struct Outcome {
    moves: usize,
    families: usize,
    dropped: Vec<Origin>,
}

pub(super) fn select(
    blocks: &[(DraftBlockId, DraftBlock)],
    entry: DraftBlockId,
    layouts: &mut HashMap<DraftBlockId, BlockLayout>,
    context: &mut LoweringContext,
) {
    let transitions = Transitions::new(blocks);
    for (id, block) in blocks {
        if *id == entry || !transitions.match_targets.contains(id) {
            continue;
        }
        let original = &layouts[id];
        let parameters = inherited_first(original);
        if parameters == original.parameters {
            continue;
        }
        let candidate = block_layout(original.id, block, parameters, context);
        // This is a per-family bijection. The first-use routing contract fixes
        // clone count at output arity minus distinct inputs, independent of order.
        // Discard order can change, so compare it for every affected transition.
        let mut before = (0, 0);
        let mut after = (0, 0);
        let mut preserves_release = true;
        for &index in &transitions.affected[id] {
            let edge = &transitions.edges[index];
            let old = edge.outcome(layouts, None);
            let new = edge.outcome(layouts, Some((*id, &candidate)));
            if old.dropped != new.dropped {
                preserves_release = false;
                break;
            }
            before.0 += old.moves;
            before.1 += old.families;
            after.0 += new.moves;
            after.1 += new.families;
        }
        if preserves_release && after < before {
            layouts.insert(*id, candidate);
        }
    }
}

// These scalar columns cannot own user payloads, captures, or work. Reordering
// their parameters is also safe when the whole environment is dropped at an
// instruction error, source stop, return, or cancellation. Other columns stay put.
fn inherited_first(layout: &BlockLayout) -> Vec<ParameterSource> {
    let mut positions = BTreeMap::<_, Vec<_>>::new();
    for (index, param) in layout.params.iter().enumerate() {
        if let Some(slot) = param.local().storage_slot()
            && matches!(
                slot.family,
                StorageFamily::Int
                    | StorageFamily::Float
                    | StorageFamily::String
                    | StorageFamily::BitArray
                    | StorageFamily::UtfCodepoint
                    | StorageFamily::Bool
            )
        {
            positions.entry(slot.family).or_default().push(index);
        }
    }
    let mut parameters = layout.parameters.clone();
    for indices in positions.values() {
        let inherited = indices
            .iter()
            .filter(|&&index| matches!(layout.parameters[index], ParameterSource::Inherited(_)));
        let explicit = indices
            .iter()
            .filter(|&&index| matches!(layout.parameters[index], ParameterSource::Explicit(_)));
        for (&destination, &source) in indices.iter().zip(inherited.chain(explicit)) {
            parameters[destination] = layout.parameters[source].clone();
        }
    }
    parameters
}

impl<'draft> Transitions<'draft> {
    fn new(blocks: &'draft [(DraftBlockId, DraftBlock)]) -> Self {
        let mut transitions = Self {
            edges: Vec::new(),
            affected: HashMap::new(),
            match_targets: HashSet::new(),
        };
        for &(id, ref block) in blocks {
            let mut add = |inputs: Inputs<'draft>| {
                let index = transitions.edges.len();
                transitions.affected.entry(id).or_default().push(index);
                if let Some(target) = inputs.target() {
                    transitions.affected.entry(target).or_default().push(index);
                }
                if let Inputs::Match(edge) = inputs {
                    transitions.match_targets.insert(edge.target);
                }
                transitions.edges.push(Transition {
                    source: id,
                    block,
                    inputs,
                });
            };
            match &block.terminator {
                DraftTerminator::Jump(edge) => add(Inputs::Edge(edge)),
                DraftTerminator::BoolBranch { true_, false_, .. } => {
                    add(Inputs::Edge(true_));
                    add(Inputs::Edge(false_));
                }
                DraftTerminator::IntSwitch {
                    clauses, fallback, ..
                } => {
                    for (_, edge) in clauses {
                        add(Inputs::Edge(edge));
                    }
                    add(Inputs::Edge(fallback));
                }
                DraftTerminator::FloatSwitch {
                    clauses, fallback, ..
                } => {
                    for (_, edge) in clauses {
                        add(Inputs::Edge(edge));
                    }
                    add(Inputs::Edge(fallback));
                }
                DraftTerminator::StringSwitch {
                    clauses, fallback, ..
                } => {
                    for (_, edge) in clauses {
                        add(Inputs::Edge(edge));
                    }
                    add(Inputs::Edge(fallback));
                }
                DraftTerminator::Match {
                    success, failure, ..
                } => {
                    add(Inputs::Match(success));
                    add(Inputs::Edge(failure));
                }
                DraftTerminator::Echo { next, .. } => add(Inputs::Edge(next)),
                DraftTerminator::TailCall { args, .. }
                | DraftTerminator::NeverCall { args, .. } => add(Inputs::Call(args)),
                DraftTerminator::Return { .. }
                | DraftTerminator::SourceStop { .. }
                | DraftTerminator::LetAssertPanic { .. } => {}
            }
        }
        for edges in transitions.affected.values_mut() {
            edges.dedup();
        }
        transitions
    }
}

impl Inputs<'_> {
    fn target(&self) -> Option<DraftBlockId> {
        match self {
            Self::Edge(edge) => Some(edge.target),
            Self::Match(edge) => Some(edge.target),
            Self::Call(_) => None,
        }
    }
}

impl Transition<'_> {
    fn outcome(
        &self,
        layouts: &HashMap<DraftBlockId, BlockLayout>,
        replacement: Option<(DraftBlockId, &BlockLayout)>,
    ) -> Outcome {
        let layout = |id| match replacement {
            Some((changed, candidate)) if changed == id => candidate,
            _ => &layouts[&id],
        };
        let source = layout(self.source);
        let mut values = BTreeMap::<_, Vec<_>>::new();
        for (parameter, slot) in source.parameters.iter().zip(&source.params) {
            if let Some(slot) = slot.local().storage_slot() {
                values
                    .entry(slot.family)
                    .or_default()
                    .push(Origin::Local(parameter.value(self.block).key));
            }
        }
        for instruction in &self.block.instructions {
            let output = instruction.output();
            if let Some(slot) = source.values.any(&output).storage_slot() {
                values
                    .entry(slot.family)
                    .or_default()
                    .push(Origin::Local(output.key));
            }
        }
        let transfer = match self.inputs {
            Inputs::Edge(edge) => {
                let args = edge_arguments(edge, source, layout(edge.target));
                transfer::arguments(source, &args)
            }
            Inputs::Match(edge) => {
                let target = layout(edge.target);
                let args = match_arguments(edge, source, target);
                let mut bindings = BTreeMap::new();
                for (arg, param) in args.iter().zip(&target.params) {
                    if let MatchEdgeArgument::Binding(index) = arg
                        && let Some(slot) = param.local().storage_slot()
                    {
                        bindings.insert(*index, slot.family);
                    }
                }
                for (index, family) in bindings {
                    values
                        .entry(family)
                        .or_default()
                        .push(Origin::Binding(index));
                }
                transfer::matched(source, &args, &target.params).1
            }
            Inputs::Call(args) => transfer::arguments(source, &source.values.any_slice(args)),
        };
        let mut outcome = Outcome {
            moves: transfer
                .families
                .iter()
                .map(|route| route.steps.len())
                .sum(),
            families: transfer.families.len(),
            dropped: Vec::new(),
        };
        let mut routes = transfer.families.iter().peekable();
        for (family, mut values) in values {
            let Some(route) = routes.next_if(|route| route.family == family) else {
                continue;
            };
            for &TransferStep {
                source,
                destination,
            } in route.steps.iter()
            {
                if source < destination {
                    values.push(values[source]);
                    let last = values.len() - 1;
                    values.swap(last, destination);
                } else {
                    values.swap(source, destination);
                }
            }
            outcome.dropped.extend(values.drain(route.length..));
        }
        outcome
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ParameterSource, block_layout, reachable_blocks};
    use super::{Origin, Transitions, inherited_first, select};
    use crate::plan::FunctionTemplateId;
    use crate::plan::execution::graph::{BlockId, MatchEdgeArgument, Terminator};
    use crate::plan::execution::lowering::graph::draft::instruction::DraftIntInstruction;
    use crate::plan::execution::lowering::graph::draft::pattern::{
        DraftMatchPattern, DraftMatchPatternBinding,
    };
    use crate::plan::execution::lowering::graph::draft::{DraftGraphBuilder, DraftInt};
    use crate::plan::execution::lowering::graph::liveness::GraphLiveness;
    use crate::plan::execution::lowering::local::{LocalKey, LocalKind};
    use crate::plan::execution::lowering::specialization::{
        RepresentationContext, SpecializationKey, SpecializedFunctionShape, SpecializedValueShape,
        StoredValueShape,
    };
    use crate::plan::execution::lowering::{LoweringContext, ProgramConstantTemplates};
    use crate::{ExecutionPlan, Value, compile_typed_module, plan_module, run_main};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn source_assertion_keeps_inherited_values_before_the_new_binding_deterministically() {
        let source = r#"
pub fn main() {
  let keep = 40
  let captured = fn(value) { keep + value }
  let held = #(captured, [1, 2])
  let assert [head, ..] = [2]
  echo head
  held.0(head) + keep - 40
}
"#;
        let mut explanations = Vec::new();
        for _ in 0..2 {
            let typed = compile_typed_module("main", "main.gleam", source).unwrap();
            let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
            let mut matched = Vec::new();
            for function in plan.program.functions.value_returns.int_functions.iter() {
                for block in function.body().block_graph().blocks() {
                    if let Terminator::Match(match_) = block.terminator() {
                        matched.push(
                            match_
                                .success()
                                .args()
                                .iter()
                                .map(|arg| match arg {
                                    MatchEdgeArgument::Binding(index) => Some(*index),
                                    MatchEdgeArgument::Value(_) => None,
                                })
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            assert_eq!(matched, [vec![None, Some(0), None]]);
            let mut echo = Vec::new();
            assert_eq!(run_main(&plan, &mut echo), Ok(Value::Int(42.into())));
            assert_eq!(
                echo.iter()
                    .map(|output| output.value().inspect().to_string())
                    .collect::<Vec<_>>(),
                ["2"]
            );
            explanations.push(plan.explain().to_string());
        }
        assert_eq!(explanations[0], explanations[1]);
    }

    #[test]
    fn moves_only_scalar_parameters_and_preserves_the_other_storage_columns() {
        let shapes = [
            (StoredValueShape::Int, true),
            (StoredValueShape::Float, true),
            (StoredValueShape::String, true),
            (StoredValueShape::BitArray, true),
            (StoredValueShape::UtfCodepoint, true),
            (StoredValueShape::Bool, true),
            (StoredValueShape::Nil, false),
            (
                StoredValueShape::List(Box::new(SpecializedValueShape::Int)),
                false,
            ),
            (
                StoredValueShape::Tuple(vec![SpecializedValueShape::Int].into()),
                false,
            ),
            (
                StoredValueShape::Function(Box::new(SpecializedFunctionShape::new(
                    vec![],
                    SpecializedValueShape::Int,
                ))),
                false,
            ),
        ];
        let entry_keys = shapes
            .iter()
            .enumerate()
            .map(|(index, (shape, _))| (LocalKey::new(LocalKind::Generic, index), shape.clone()))
            .collect();
        let (mut draft, entry) = DraftGraphBuilder::<DraftInt, usize>::new(entry_keys, vec![]);
        let explicit = shapes
            .iter()
            .map(|(shape, _)| draft.value_ref(shape.clone()))
            .collect::<Vec<_>>();
        let inherited = (0..shapes.len())
            .map(|index| entry.scope().get(LocalKey::new(LocalKind::Generic, index)))
            .collect::<Vec<_>>();
        let target = draft.block(entry.scope().clone(), explicit.clone());
        let target_id = target.id();
        draft.finish_tail_call(
            target,
            0,
            explicit.iter().chain(&inherited).cloned().collect(),
        );
        draft.finish_jump(entry, target_id, inherited.clone());
        let block = &draft.graph().blocks[&target_id];
        let parameters = (0..shapes.len())
            .map(ParameterSource::Explicit)
            .chain(inherited.iter().cloned().map(ParameterSource::Inherited))
            .collect();
        let mut context = context();
        let original = block_layout(BlockId::new(1), block, parameters, &mut context);
        let ordered = inherited_first(&original);
        let candidate = block_layout(original.id, block, ordered, &mut context);
        for (index, (_, movable)) in shapes.iter().enumerate() {
            let first = candidate.parameters[index].value(block);
            let second = candidate.parameters[index + shapes.len()].value(block);
            let expected = if *movable {
                (&inherited[index], &explicit[index])
            } else {
                (&explicit[index], &inherited[index])
            };
            assert_eq!((first.key, second.key), (expected.0.key, expected.1.key));
            assert_eq!(
                candidate
                    .values
                    .any(&explicit[index])
                    .storage_slot()
                    .map(|slot| slot.index),
                if *movable {
                    Some(1)
                } else {
                    original
                        .values
                        .any(&explicit[index])
                        .storage_slot()
                        .map(|slot| slot.index)
                }
            );
            assert_eq!(
                candidate.params[index].shape(),
                original.params[index].shape()
            );
        }
    }

    #[test]
    fn selects_a_cheaper_match_layout_but_keeps_drop_order_and_nonimproving_layouts() {
        // The three exits respectively prefer inherited-first, have equal cost,
        // and would reverse the two discarded parameters after computing a sum.
        for scenario in 0..3 {
            let (mut draft, mut entry) = DraftGraphBuilder::<DraftInt, usize>::new(vec![], vec![]);
            let inherited =
                draft.int_instruction(&mut entry, DraftIntInstruction::Value(10.into()));
            let subject = draft.int_instruction(&mut entry, DraftIntInstruction::Value(20.into()));
            let binding = draft.value_ref(StoredValueShape::Int);
            let mut success = draft.block(entry.scope().clone(), vec![binding.clone()]);
            let success_id = success.id();
            let failure = draft.empty_block(entry.scope().clone());
            let failure_id = failure.id();
            let args = match scenario {
                0 => vec![inherited.erase(), binding.clone(), inherited.erase()],
                1 => vec![binding.clone(), inherited.erase()],
                _ => {
                    let sum = draft.int_instruction(
                        &mut success,
                        DraftIntInstruction::Add {
                            left: DraftInt::from_ref(&binding),
                            right: inherited.clone(),
                        },
                    );
                    vec![sum.erase()]
                }
            };
            draft.finish_tail_call(success, 0, args);
            draft.finish_return(failure, inherited.clone());
            draft.finish_match(
                entry,
                subject.erase(),
                DraftMatchPattern::Bind(DraftMatchPatternBinding {
                    value: binding.clone(),
                    index: 0,
                }),
                success_id,
                1,
                failure_id,
            );
            let liveness = GraphLiveness::analyze(draft.graph());
            let order = reachable_blocks(draft.graph());
            let entry_id = draft.graph().entry;
            let mut blocks = draft.graph.blocks.into_iter().collect::<HashMap<_, _>>();
            let blocks = order
                .into_iter()
                .map(|id| (id, blocks.remove(&id).unwrap()))
                .collect::<Vec<_>>();
            let mut context = context();
            let mut layouts = blocks
                .iter()
                .enumerate()
                .map(|(index, (id, block))| {
                    let parameters = liveness
                        .explicit_params(*id)
                        .iter()
                        .copied()
                        .map(ParameterSource::Explicit)
                        .chain(
                            liveness
                                .inherited(*id)
                                .iter()
                                .cloned()
                                .map(ParameterSource::Inherited),
                        )
                        .collect();
                    (
                        *id,
                        block_layout(BlockId::new(index), block, parameters, &mut context),
                    )
                })
                .collect::<HashMap<_, _>>();
            let transitions = Transitions::new(&blocks);
            let relevant = &transitions.affected[&success_id];
            assert_eq!(relevant.len(), 2);
            let before = relevant
                .iter()
                .map(|&index| transitions.edges[index].outcome(&layouts, None))
                .collect::<Vec<_>>();
            assert_eq!(before[0].dropped, [Origin::Local(subject.erase().key)]);
            let block = &blocks[1].1;
            let candidate = block_layout(
                layouts[&success_id].id,
                block,
                inherited_first(&layouts[&success_id]),
                &mut context,
            );
            let proposed = relevant
                .iter()
                .map(|&index| {
                    transitions.edges[index].outcome(&layouts, Some((success_id, &candidate)))
                })
                .collect::<Vec<_>>();
            let moves = |outcomes: &[super::Outcome]| {
                outcomes.iter().map(|outcome| outcome.moves).sum::<usize>()
            };
            match scenario {
                0 => {
                    assert_eq!((moves(&before), moves(&proposed)), (4, 2));
                }
                1 => {
                    assert_eq!((moves(&before), moves(&proposed)), (2, 2));
                }
                _ => {
                    assert_eq!(
                        before[1].dropped,
                        [
                            Origin::Local(inherited.erase().key),
                            Origin::Local(binding.key)
                        ]
                    );
                    assert_eq!(
                        proposed[1].dropped,
                        [
                            Origin::Local(binding.key),
                            Origin::Local(inherited.erase().key)
                        ]
                    );
                }
            }
            select(&blocks, entry_id, &mut layouts, &mut context);
            let keys = layouts[&success_id]
                .parameters
                .iter()
                .map(|source| source.value(block).key)
                .collect::<Vec<_>>();
            assert_eq!(
                keys,
                if scenario == 0 {
                    vec![inherited.erase().key, binding.key]
                } else {
                    vec![binding.key, inherited.erase().key]
                }
            );
            for (&index, old) in relevant.iter().zip(before) {
                assert_eq!(
                    transitions.edges[index].outcome(&layouts, None).dropped,
                    old.dropped
                );
            }
        }
    }

    #[test]
    fn indexes_many_predecessors_once_and_remaps_a_wide_shared_target() {
        for (fan_in, width) in [(2usize, 2usize), (64, 128)] {
            let keys = (0..width)
                .map(|index| (LocalKey::new(LocalKind::Int, index), StoredValueShape::Int))
                .collect();
            let (mut draft, entry) = DraftGraphBuilder::<DraftInt, usize>::new(keys, vec![]);
            let inherited = (0..width)
                .map(|index| entry.scope().get(LocalKey::new(LocalKind::Int, index)))
                .collect::<Vec<_>>();
            let binding = draft.value_ref(StoredValueShape::Int);
            let target = draft.block(entry.scope().clone(), vec![binding.clone()]);
            let target_id = target.id();
            let failure = draft.empty_block(entry.scope().clone());
            let failure_id = failure.id();
            draft.finish_return(failure, DraftInt::from_ref(&inherited[0]));
            draft.finish_tail_call(
                target,
                0,
                inherited.iter().cloned().chain([binding.clone()]).collect(),
            );
            let mut clauses = Vec::new();
            for index in 0..fan_in {
                let predecessor = draft.empty_block(entry.scope().clone());
                clauses.push((index.into(), predecessor.id()));
                draft.finish_match(
                    predecessor,
                    inherited[0].clone(),
                    DraftMatchPattern::Bind(DraftMatchPatternBinding {
                        value: binding.clone(),
                        index: 0,
                    }),
                    target_id,
                    1,
                    failure_id,
                );
            }
            draft.finish_int_switch(
                entry,
                DraftInt::from_ref(&inherited[0]),
                clauses,
                failure_id,
            );
            let liveness = GraphLiveness::analyze(draft.graph());
            let entry_id = draft.graph().entry;
            let order = reachable_blocks(draft.graph());
            let mut blocks = draft.graph.blocks;
            let blocks = order
                .into_iter()
                .map(|id| (id, blocks.remove(&id).unwrap()))
                .collect::<Vec<_>>();
            let mut context = context();
            let mut layouts = blocks
                .iter()
                .enumerate()
                .map(|(index, (id, block))| {
                    let parameters = liveness
                        .explicit_params(*id)
                        .iter()
                        .copied()
                        .map(ParameterSource::Explicit)
                        .chain(
                            liveness
                                .inherited(*id)
                                .iter()
                                .cloned()
                                .map(ParameterSource::Inherited),
                        )
                        .collect();
                    (
                        *id,
                        block_layout(BlockId::new(index), block, parameters, &mut context),
                    )
                })
                .collect::<HashMap<_, _>>();
            let transitions = Transitions::new(&blocks);
            assert_eq!(transitions.edges.len(), 3 * fan_in + 2);
            assert_eq!(transitions.affected[&target_id].len(), fan_in + 1);
            assert_eq!(
                transitions.affected.values().map(Vec::len).sum::<usize>(),
                2 * transitions.edges.len() - 1
            );
            let entry_keys = layouts[&entry_id]
                .parameters
                .iter()
                .map(|source| source.value(&blocks[0].1).key)
                .collect::<Vec<_>>();
            select(&blocks, entry_id, &mut layouts, &mut context);
            let target_block = &blocks.iter().find(|(id, _)| *id == target_id).unwrap().1;
            assert_eq!(
                layouts[&target_id]
                    .parameters
                    .iter()
                    .map(|source| source.value(target_block).key)
                    .collect::<Vec<_>>(),
                inherited
                    .iter()
                    .map(|value| value.key)
                    .chain([binding.key])
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                layouts[&entry_id]
                    .parameters
                    .iter()
                    .map(|source| source.value(&blocks[0].1).key)
                    .collect::<Vec<_>>(),
                entry_keys
            );
            for edge in &transitions.edges {
                let outcome = edge.outcome(&layouts, None);
                if edge.inputs.target() == Some(target_id) {
                    assert_eq!((outcome.moves, outcome.families), (0, 0));
                    assert!(outcome.dropped.is_empty());
                }
            }
        }
    }

    fn context() -> LoweringContext {
        LoweringContext::new(
            HashMap::new(),
            RepresentationContext::new(vec![]),
            ProgramConstantTemplates { modules: vec![] },
            SpecializationKey::monomorphic(FunctionTemplateId::new(0)),
            HashSet::new(),
        )
    }
}
