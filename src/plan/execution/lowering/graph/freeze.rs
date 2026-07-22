mod instruction;
mod pattern;
mod value;

use super::draft::{
    DraftBlock, DraftBlockId, DraftEdge, DraftGraph, DraftGraphBuilder, DraftGraphValue,
    DraftInstruction, DraftMatchEdge, DraftMatchEdgeArgument, DraftNeverCallTarget,
    DraftTerminator, DraftValueRef, LoweredFunctionGraph,
};
use super::liveness::GraphLiveness;
use crate::plan::execution;
use std::collections::{HashMap, HashSet};
use value::BlockValues;
pub(in crate::plan::execution::lowering) use value::FreezeGraphValue;

struct BlockLayout {
    params: Vec<execution::ParamSlot>,
    values: BlockValues,
}

pub(super) fn freeze<Return, TailCall>(
    graph: DraftGraphBuilder<Return, TailCall>,
    context: &mut super::super::LoweringContext,
) -> LoweredFunctionGraph<execution::graph::FunctionGraph<Return::Frozen, TailCall>>
where
    Return: DraftGraphValue + FreezeGraphValue,
    TailCall: Clone,
{
    let liveness = GraphLiveness::analyze(graph.graph());
    let order = reachable_blocks(graph.graph());
    let block_ids = order
        .iter()
        .enumerate()
        .map(|(index, draft)| (*draft, execution::graph::BlockId::new(index)))
        .collect::<HashMap<_, _>>();
    let entry = graph.graph.entry;
    let parameter_count = graph.graph.parameter_count;
    let returns = graph.returns;
    let tail_calls = graph.tail_calls;
    let mut draft_blocks = graph
        .graph
        .blocks
        .into_iter()
        .filter(|(draft_id, _)| block_ids.contains_key(draft_id))
        .collect::<Vec<_>>();
    draft_blocks.sort_by_key(|(draft_id, _)| block_ids[draft_id].index());
    let mut exits = Vec::new();
    let mut blocks = Vec::with_capacity(draft_blocks.len());
    for (draft_id, block) in draft_blocks {
        let DraftBlock {
            explicit_params,
            instructions,
            terminator,
        } = block;
        let layout = block_layout(
            draft_id,
            &explicit_params,
            &instructions,
            &liveness,
            context,
        );
        let instructions = instructions
            .iter()
            .map(|draft| instruction::freeze(draft, &layout.values, context))
            .collect();
        let terminator = freeze_terminator(
            terminator,
            &returns,
            &tail_calls,
            &layout,
            &liveness,
            &block_ids,
            &mut exits,
        );
        blocks.push(execution::graph::Block::new(
            layout.params,
            instructions,
            terminator,
        ));
    }

    LoweredFunctionGraph {
        parameter_count,
        body: execution::graph::FunctionGraph::from_parts(block_ids[&entry], blocks, exits),
    }
}

fn block_layout(
    draft_id: DraftBlockId,
    explicit_params: &[DraftValueRef],
    instructions: &[DraftInstruction],
    liveness: &GraphLiveness,
    context: &mut super::super::LoweringContext,
) -> BlockLayout {
    let mut values = BlockValues::default();
    let mut params = Vec::new();
    for value in liveness
        .explicit_params(draft_id)
        .iter()
        .map(|index| &explicit_params[*index])
        .chain(liveness.inherited(draft_id))
    {
        params.push(values.allocate(value, context));
    }
    for instruction in instructions {
        values.allocate(&instruction.output(), context);
    }
    BlockLayout { params, values }
}

fn reachable_blocks(graph: &DraftGraph) -> Vec<DraftBlockId> {
    let mut order = Vec::new();
    let mut visited = HashSet::new();
    let mut pending = vec![graph.entry];
    while let Some(block) = pending.pop() {
        if !visited.insert(block) {
            continue;
        }
        order.push(block);
        let successors = graph.blocks[&block].terminator.successors();
        pending.extend(successors.into_iter().rev());
    }
    order
}

fn freeze_terminator<Return, TailCall>(
    terminator: DraftTerminator,
    returns: &[Return],
    tail_calls: &[TailCall],
    layout: &BlockLayout,
    liveness: &GraphLiveness,
    block_ids: &HashMap<DraftBlockId, execution::graph::BlockId>,
    exits: &mut Vec<execution::graph::FunctionGraphExit<Return::Frozen, TailCall>>,
) -> execution::graph::Terminator
where
    Return: DraftGraphValue + FreezeGraphValue,
    TailCall: Clone,
{
    use execution::graph::Terminator as E;

    match terminator {
        DraftTerminator::Jump(edge) => E::Jump(freeze_edge(&edge, layout, liveness, block_ids)),
        DraftTerminator::BoolBranch {
            subject,
            true_,
            false_,
        } => E::BoolBranch {
            subject: layout.values.bool(&subject),
            true_: freeze_edge(&true_, layout, liveness, block_ids),
            false_: freeze_edge(&false_, layout, liveness, block_ids),
        },
        DraftTerminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => E::IntSwitch {
            subject: layout.values.int(&subject),
            clauses: clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, liveness, block_ids)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            fallback: freeze_edge(&fallback, layout, liveness, block_ids),
        },
        DraftTerminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => E::FloatSwitch {
            subject: layout.values.float(&subject),
            clauses: clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, liveness, block_ids)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            fallback: freeze_edge(&fallback, layout, liveness, block_ids),
        },
        DraftTerminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => E::StringSwitch {
            subject: layout.values.string(&subject),
            clauses: clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, liveness, block_ids)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            fallback: freeze_edge(&fallback, layout, liveness, block_ids),
        },
        DraftTerminator::Match {
            subject,
            pattern: draft_pattern,
            success,
            failure,
        } => E::Match {
            subject: layout.values.any(&subject),
            pattern: pattern::freeze(draft_pattern, &layout.values),
            success: freeze_match_edge(&success, layout, liveness, block_ids),
            failure: freeze_edge(&failure, layout, liveness, block_ids),
        },
        DraftTerminator::Return { value: _, index } => {
            let id = execution::graph::GraphExitId::new(exits.len());
            exits.push(execution::graph::FunctionGraphExit::Return(
                returns[index].freeze(&layout.values),
            ));
            E::Exit(id)
        }
        DraftTerminator::TailCall { function, args } => {
            let id = execution::graph::GraphExitId::new(exits.len());
            exits.push(execution::graph::FunctionGraphExit::TailCall {
                function: tail_calls[function].clone(),
                args: layout.values.any_slice(&args),
            });
            E::Exit(id)
        }
        DraftTerminator::SourceStop {
            kind,
            message,
            site,
        } => E::SourceStop {
            kind,
            message: message
                .as_ref()
                .map(|message| layout.values.string(message)),
            site,
        },
        DraftTerminator::LetAssertPanic {
            subject,
            message,
            site,
            pattern_span,
        } => E::LetAssertPanic {
            subject: layout.values.any(&subject),
            message: message
                .as_ref()
                .map(|message| layout.values.string(message)),
            site,
            pattern_span,
        },
        DraftTerminator::NeverCall { function, args } => E::NeverCall {
            function: match function {
                DraftNeverCallTarget::Direct(function) => {
                    execution::graph::NeverCallTarget::Direct(function)
                }
                DraftNeverCallTarget::Value(function) => execution::graph::NeverCallTarget::Value(
                    layout.values.never_function(&function),
                ),
            },
            args: layout.values.any_slice(&args),
        },
    }
}

fn freeze_edge(
    edge: &DraftEdge,
    source: &BlockLayout,
    liveness: &GraphLiveness,
    block_ids: &HashMap<DraftBlockId, execution::graph::BlockId>,
) -> execution::graph::Edge {
    let mut args = liveness
        .explicit_params(edge.target)
        .iter()
        .map(|index| source.values.any(&edge.explicit_args[*index]))
        .collect::<Vec<_>>();
    args.extend(
        liveness
            .inherited(edge.target)
            .iter()
            .map(|value| source.values.any(value)),
    );
    let target = block_ids[&edge.target];
    execution::graph::Edge::new(target, args)
}

fn freeze_match_edge(
    edge: &DraftMatchEdge,
    source: &BlockLayout,
    liveness: &GraphLiveness,
    block_ids: &HashMap<DraftBlockId, execution::graph::BlockId>,
) -> execution::graph::MatchEdge {
    let mut args = liveness
        .explicit_params(edge.target)
        .iter()
        .map(|index| match &edge.explicit_args[*index] {
            DraftMatchEdgeArgument::Binding(index) => {
                execution::graph::MatchEdgeArgument::Binding(*index)
            }
        })
        .collect::<Vec<_>>();
    args.extend(
        liveness
            .inherited(edge.target)
            .iter()
            .map(|value| execution::graph::MatchEdgeArgument::Value(source.values.any(value))),
    );
    let target = block_ids[&edge.target];
    execution::graph::MatchEdge::new(target, args)
}
