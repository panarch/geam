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
use std::convert::Infallible;
use value::BlockValues;
pub(in crate::plan::execution::lowering) use value::FreezeGraphValue;

struct BlockLayout {
    params: Vec<execution::graph::ParamSlot>,
    values: BlockValues,
}

struct FrozenGraph<Return, TailCall> {
    graph: execution::graph::BlockGraph,
    exits: Vec<FrozenGraphExit<Return, TailCall>>,
}

enum FrozenGraphExit<Return, TailCall> {
    Return(Return),
    TailCall {
        function: TailCall,
        args: Box<[execution::graph::ParamLocal]>,
    },
}

pub(super) fn freeze<Return, TailCall>(
    graph: DraftGraphBuilder<Return, TailCall>,
    context: &mut super::super::LoweringContext,
) -> LoweredFunctionGraph<execution::function::FunctionBody<Return::Frozen, TailCall>>
where
    Return: DraftGraphValue + FreezeGraphValue,
    TailCall: Clone,
{
    freeze_graph(graph, context).map(|frozen| {
        let exits = frozen
            .exits
            .into_iter()
            .map(|exit| match exit {
                FrozenGraphExit::Return(value) => execution::function::FunctionExit::Return(value),
                FrozenGraphExit::TailCall { function, args } => {
                    execution::function::FunctionExit::TailCall { function, args }
                }
            })
            .collect();
        execution::function::FunctionBody::from_parts(frozen.graph, exits)
    })
}

pub(super) fn freeze_constant<Return>(
    graph: DraftGraphBuilder<Return, Infallible>,
    context: &mut super::super::LoweringContext,
) -> execution::constant::ConstantProgram<Return::Frozen>
where
    Return: DraftGraphValue + FreezeGraphValue,
{
    let frozen = freeze_graph(graph, context).body;
    let returns = frozen
        .exits
        .into_iter()
        .map(|exit| match exit {
            FrozenGraphExit::Return(value) => value,
            FrozenGraphExit::TailCall { function, .. } => match function {},
        })
        .collect();
    execution::constant::ConstantProgram::from_parts(frozen.graph, returns)
}

fn freeze_graph<Return, TailCall>(
    graph: DraftGraphBuilder<Return, TailCall>,
    context: &mut super::super::LoweringContext,
) -> LoweredFunctionGraph<FrozenGraph<Return::Frozen, TailCall>>
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
        body: FrozenGraph {
            graph: execution::graph::BlockGraph::from_parts(block_ids[&entry], blocks),
            exits,
        },
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
    exits: &mut Vec<FrozenGraphExit<Return::Frozen, TailCall>>,
) -> execution::graph::Terminator
where
    Return: DraftGraphValue + FreezeGraphValue,
    TailCall: Clone,
{
    use execution::graph::Terminator as E;

    match terminator {
        DraftTerminator::Jump(edge) => E::Jump(execution::graph::Jump::new(freeze_edge(
            &edge, layout, liveness, block_ids,
        ))),
        DraftTerminator::BoolBranch {
            subject,
            true_,
            false_,
        } => E::BoolBranch(execution::graph::BoolBranch::new(
            layout.values.bool(&subject),
            freeze_edge(&true_, layout, liveness, block_ids),
            freeze_edge(&false_, layout, liveness, block_ids),
        )),
        DraftTerminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => E::IntSwitch(execution::graph::IntSwitch::new(
            layout.values.int(&subject),
            clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, liveness, block_ids)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            freeze_edge(&fallback, layout, liveness, block_ids),
        )),
        DraftTerminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => E::FloatSwitch(execution::graph::FloatSwitch::new(
            layout.values.float(&subject),
            clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, liveness, block_ids)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            freeze_edge(&fallback, layout, liveness, block_ids),
        )),
        DraftTerminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => E::StringSwitch(execution::graph::StringSwitch::new(
            layout.values.string(&subject),
            clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, liveness, block_ids)))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            freeze_edge(&fallback, layout, liveness, block_ids),
        )),
        DraftTerminator::Match {
            subject,
            pattern: draft_pattern,
            success,
            failure,
        } => E::Match(execution::graph::Match::new(
            layout.values.any(&subject),
            pattern::freeze(draft_pattern, &layout.values),
            freeze_match_edge(&success, layout, liveness, block_ids),
            freeze_edge(&failure, layout, liveness, block_ids),
        )),
        DraftTerminator::Echo {
            subject,
            message,
            site,
            next,
        } => E::Echo(execution::graph::Echo::new(
            layout.values.any(&subject),
            message
                .as_ref()
                .map(|message| layout.values.string(message)),
            site,
            freeze_edge(&next, layout, liveness, block_ids),
        )),
        DraftTerminator::Return { value: _, index } => {
            let id = execution::graph::BlockGraphExitId::new(exits.len());
            exits.push(FrozenGraphExit::Return(
                returns[index].freeze(&layout.values),
            ));
            E::Exit(id)
        }
        DraftTerminator::TailCall { function, args } => {
            let id = execution::graph::BlockGraphExitId::new(exits.len());
            exits.push(FrozenGraphExit::TailCall {
                function: tail_calls[function].clone(),
                args: layout.values.any_slice(&args),
            });
            E::Exit(id)
        }
        DraftTerminator::SourceStop {
            kind,
            message,
            site,
        } => E::SourceStop(execution::graph::SourceStop::new(
            kind,
            message
                .as_ref()
                .map(|message| layout.values.string(message)),
            site,
        )),
        DraftTerminator::LetAssertPanic {
            subject,
            message,
            site,
            pattern_span,
        } => E::LetAssertPanic(execution::graph::LetAssertPanic::new(
            layout.values.any(&subject),
            message
                .as_ref()
                .map(|message| layout.values.string(message)),
            site,
            pattern_span,
        )),
        DraftTerminator::NeverCall {
            function,
            args,
            site,
        } => E::NeverCall(execution::graph::NeverCall::new(
            match function {
                DraftNeverCallTarget::Direct(function) => {
                    execution::graph::NeverCallTarget::Direct(function)
                }
                DraftNeverCallTarget::Value(function) => execution::graph::NeverCallTarget::Value(
                    layout.values.never_function(&function),
                ),
            },
            layout.values.any_slice(&args),
            site,
        )),
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

#[cfg(test)]
mod tests {
    use super::super::draft::instruction::{DraftBoolInstruction, DraftIntInstruction};
    use super::super::draft::{DraftGraphBuilder, DraftInt};
    use super::freeze;
    use crate::plan::FunctionCallTarget;
    use crate::plan::execution;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::function::{
        ExecutionGraphProfile, FunctionExit, IntFunctionId, ProfiledFunctionBody,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, BoolLocalId, Edge, IntInstruction, IntLocalId, ParamLocal,
        ProfiledInstruction, ProfiledInstructionKind, Terminator,
    };
    use crate::plan::execution::lowering::specialization::{
        RepresentationContext, SpecializationKey, StoredValueShape,
    };
    use std::collections::{HashMap, HashSet};
    use std::convert::Infallible;

    type FunctionBody<Return, TailCall> = ProfiledFunctionBody<Return, TailCall, Infallible>;

    #[derive(Clone, Copy)]
    enum IntBinaryOperation {
        Add,
        Multiply,
    }

    #[test]
    fn freezes_dense_locals_merges_and_edge_arguments_in_reachable_order() {
        let plan = execution_plan(
            r#"
fn choose(flag: Bool, value: Int) -> Int {
  let selected = case flag {
    True -> value + 1
    False -> value + 2
  }
  selected * 3
}

pub fn main() { choose(True, 10) }
"#,
        );
        let function = plan.int_function(IntFunctionId(1));
        let body = function.body();
        let block_graph = body.block_graph();

        assert_eq!(block_graph.entry(), BlockId::new(0));
        assert_eq!(block_graph.blocks().len(), 4);
        assert_eq!(
            function
                .entry()
                .params(body)
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![
                &ParamLocal::Bool(BoolLocalId(0)),
                &ParamLocal::Int(IntLocalId(0)),
            ],
        );

        let entry = block_graph.block(BlockId::new(0));
        assert!(entry.instructions().is_empty());
        let (subject, true_, false_) = bool_branch(entry.terminator());
        assert_eq!(subject, BoolLocalId(0));
        assert_eq!(true_.target(), BlockId::new(1));
        assert_eq!(false_.target(), BlockId::new(3));
        assert_eq!(true_.args(), &[ParamLocal::Int(IntLocalId(0))]);
        assert_eq!(false_.args(), &[ParamLocal::Int(IntLocalId(0))]);

        assert_branch_add_and_jump(&plan, body, BlockId::new(1), 1, BlockId::new(2));

        let merge = block_graph.block(BlockId::new(2));
        assert_eq!(
            merge
                .params()
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![&ParamLocal::Int(IntLocalId(0))],
        );
        assert_int_shape(&plan, merge.params()[0].shape());
        assert_eq!(merge.instructions().len(), 2);
        assert_int_value(&plan, &merge.instructions()[0], IntLocalId(1), 3);
        let multiply = &merge.instructions()[1];
        assert_eq!(multiply.output().local(), &ParamLocal::Int(IntLocalId(2)));
        assert_int_shape(&plan, multiply.output().shape());
        assert_eq!(
            int_binary_operands(multiply, IntBinaryOperation::Multiply),
            (IntLocalId(0), IntLocalId(1)),
        );
        assert_eq!(returned_int(body, merge.terminator()), IntLocalId(2));

        assert_branch_add_and_jump(&plan, body, BlockId::new(3), 2, BlockId::new(2));
    }

    #[test]
    fn freezes_explicit_parameters_before_inherited_values_and_packs_jump_arguments() {
        let (mut draft, mut entry) =
            DraftGraphBuilder::<DraftInt, usize>::new(Vec::new(), Vec::new());
        let inherited = draft.int_instruction(&mut entry, DraftIntInstruction::Value(10.into()));
        let explicit = draft.int_instruction(&mut entry, DraftIntInstruction::Value(20.into()));
        let target_param = draft.value_ref(StoredValueShape::Int);
        let mut target = draft.block(entry.scope().clone(), vec![target_param.clone()]);
        let result = draft.int_instruction(
            &mut target,
            DraftIntInstruction::Add {
                left: DraftInt::from_ref(&target_param),
                right: inherited.clone(),
            },
        );
        let target_id = target.id();
        draft.finish_return(target, result);
        draft.finish_jump(entry, target_id, vec![explicit.erase()]);

        let lowered = freeze(draft, &mut lowering_context());
        assert_eq!(lowered.parameter_count, 0);
        let graph = lowered.body.block_graph();
        assert_eq!(graph.blocks().len(), 2);
        assert_eq!(
            jump(graph.block(BlockId::new(0)).terminator()).args(),
            &[
                ParamLocal::Int(IntLocalId(1)),
                ParamLocal::Int(IntLocalId(0)),
            ],
        );

        let target = graph.block(BlockId::new(1));
        assert_eq!(
            target
                .params()
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![
                &ParamLocal::Int(IntLocalId(0)),
                &ParamLocal::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(
            int_binary_operands(&target.instructions()[0], IntBinaryOperation::Add),
            (IntLocalId(0), IntLocalId(1)),
        );
        assert_eq!(
            returned_int(&lowered.body, target.terminator()),
            IntLocalId(2)
        );
    }

    #[test]
    fn prunes_dead_blocks_and_assigns_exits_in_reachable_block_order() {
        let (mut draft, mut entry) =
            DraftGraphBuilder::<DraftInt, usize>::new(Vec::new(), Vec::new());
        let condition = draft.bool_instruction(&mut entry, DraftBoolInstruction::Value(true));
        let scope = entry.scope().clone();
        let mut return_block = draft.empty_block(scope.clone());
        let mut tail_call_block = draft.empty_block(scope.clone());
        let mut dead_block = draft.empty_block(scope);
        let return_id = return_block.id();
        let tail_call_id = tail_call_block.id();

        let dead_value =
            draft.int_instruction(&mut dead_block, DraftIntInstruction::Value(99.into()));
        draft.finish_return(dead_block, dead_value);
        let return_value =
            draft.int_instruction(&mut return_block, DraftIntInstruction::Value(1.into()));
        draft.finish_return(return_block, return_value);
        let tail_arg =
            draft.int_instruction(&mut tail_call_block, DraftIntInstruction::Value(2.into()));
        draft.finish_tail_call(tail_call_block, 7, vec![tail_arg.erase()]);
        draft.finish_bool_branch(entry, condition, return_id, tail_call_id);

        let lowered = freeze(draft, &mut lowering_context());
        let graph = lowered.body.block_graph();
        assert_eq!(graph.blocks().len(), 3);
        let (_, true_, false_) = bool_branch(graph.block(BlockId::new(0)).terminator());
        assert_eq!(true_.target(), BlockId::new(1));
        assert_eq!(false_.target(), BlockId::new(2));

        let return_exit = exit_id(graph.block(BlockId::new(1)).terminator());
        let tail_exit = exit_id(graph.block(BlockId::new(2)).terminator());
        assert_eq!(return_exit, BlockGraphExitId::new(0));
        assert_eq!(tail_exit, BlockGraphExitId::new(1));
        assert_eq!(returned_exit(lowered.body.exit(return_exit)), IntLocalId(0));
        assert_eq!(
            tail_call_exit(lowered.body.exit(tail_exit)),
            (7, &[ParamLocal::Int(IntLocalId(0))][..]),
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a Bool branch")]
    fn bool_branch_guard_rejects_an_exit() {
        bool_branch(&Terminator::Exit(BlockGraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a jump terminator")]
    fn jump_guard_rejects_an_exit() {
        jump(&Terminator::Exit(BlockGraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain an exit terminator")]
    fn exit_guard_rejects_a_jump() {
        exit_id(&Terminator::Jump(crate::plan::execution::graph::Jump::new(
            Edge::new(BlockId::new(0), Vec::new()),
        )));
    }

    #[test]
    #[should_panic(expected = "fixture should return an Int local")]
    fn returned_int_guard_rejects_a_source_stop() {
        let plan = execution_plan("pub fn main() { 1 }");
        let body = plan.int_function(IntFunctionId(0)).body();
        returned_int(
            body,
            &Terminator::SourceStop(crate::plan::execution::graph::SourceStop::new(
                crate::plan::execution::graph::SourceStopKind::Panic,
                None,
                crate::plan::PanicSite::unknown(),
            )),
        );
    }

    #[test]
    #[should_panic(expected = "fixture should return an Int local")]
    fn returned_int_guard_rejects_a_tail_call() {
        let plan = execution_plan(
            r#"
fn loop(value: Int) -> Int { loop(value) }
pub fn main() { loop(1) }
"#,
        );
        let body = plan.int_function(IntFunctionId(0)).body();
        let graph = body.block_graph();
        returned_int(body, graph.block(graph.entry()).terminator());
    }

    #[test]
    #[should_panic(expected = "fixture should contain a return exit")]
    fn returned_exit_guard_rejects_a_tail_call() {
        returned_exit(&FunctionExit::TailCall {
            function: 0,
            args: Box::new([]),
        });
    }

    #[test]
    #[should_panic(expected = "fixture should contain a tail-call exit")]
    fn tail_call_exit_guard_rejects_a_return() {
        tail_call_exit(&FunctionExit::Return(IntLocalId(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain the requested Int binary instruction")]
    fn int_binary_guard_rejects_a_value_instruction() {
        let plan = execution_plan("pub fn main() { 1 }");
        let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        int_binary_operands(
            &graph.block(graph.entry()).instructions()[0],
            IntBinaryOperation::Add,
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain an Int value instruction")]
    fn int_value_guard_rejects_an_add_instruction() {
        let plan = execution_plan("pub fn main() { 1 + 2 }");
        let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        int_value(&graph.block(graph.entry()).instructions()[2]);
    }

    fn assert_branch_add_and_jump(
        plan: &ExecutionPlan,
        body: &FunctionBody<IntLocalId, FunctionCallTarget<IntFunctionId>>,
        block_id: BlockId,
        addend: i64,
        target: BlockId,
    ) {
        let block = body.block_graph().block(block_id);
        assert_eq!(
            block
                .params()
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![&ParamLocal::Int(IntLocalId(0))],
        );
        assert_int_shape(plan, block.params()[0].shape());
        assert_eq!(block.instructions().len(), 2);
        assert_int_value(plan, &block.instructions()[0], IntLocalId(1), addend);

        let add = &block.instructions()[1];
        assert_eq!(add.output().local(), &ParamLocal::Int(IntLocalId(2)));
        assert_int_shape(plan, add.output().shape());
        assert_eq!(
            int_binary_operands(add, IntBinaryOperation::Add),
            (IntLocalId(0), IntLocalId(1)),
        );

        let edge = jump(block.terminator());
        assert_eq!(edge.target(), target);
        assert_eq!(edge.args(), &[ParamLocal::Int(IntLocalId(2))]);
    }

    fn assert_int_value<Graph: ExecutionGraphProfile>(
        plan: &ExecutionPlan,
        instruction: &ProfiledInstruction<Graph>,
        output: IntLocalId,
        value: i64,
    ) {
        assert_eq!(instruction.output().local(), &ParamLocal::Int(output));
        assert_int_shape(plan, instruction.output().shape());
        assert_eq!(int_value(instruction), &value.into());
    }

    fn bool_branch(terminator: &Terminator) -> (BoolLocalId, &Edge, &Edge) {
        match terminator {
            Terminator::BoolBranch(branch) => (branch.subject(), branch.true_(), branch.false_()),
            _ => panic!("fixture should contain a Bool branch"),
        }
    }

    fn jump(terminator: &Terminator) -> &Edge {
        match terminator {
            Terminator::Jump(jump) => jump.edge(),
            _ => panic!("fixture should contain a jump terminator"),
        }
    }

    fn exit_id(terminator: &Terminator) -> BlockGraphExitId {
        match terminator {
            Terminator::Exit(exit) => *exit,
            _ => panic!("fixture should contain an exit terminator"),
        }
    }

    fn int_binary_operands<Graph: ExecutionGraphProfile>(
        instruction: &ProfiledInstruction<Graph>,
        operation: IntBinaryOperation,
    ) -> (IntLocalId, IntLocalId) {
        match (operation, instruction.kind()) {
            (
                IntBinaryOperation::Add,
                ProfiledInstructionKind::Int(IntInstruction::Add { left, right }),
            )
            | (
                IntBinaryOperation::Multiply,
                ProfiledInstructionKind::Int(IntInstruction::Mult { left, right }),
            ) => (*left, *right),
            _ => panic!("fixture should contain the requested Int binary instruction"),
        }
    }

    fn int_value<Graph: ExecutionGraphProfile>(
        instruction: &ProfiledInstruction<Graph>,
    ) -> &num_bigint::BigInt {
        match instruction.kind() {
            ProfiledInstructionKind::Int(IntInstruction::Value(value)) => value,
            _ => panic!("fixture should contain an Int value instruction"),
        }
    }

    fn returned_int<TailCall, Graph: ExecutionGraphProfile>(
        body: &ProfiledFunctionBody<IntLocalId, TailCall, Graph>,
        terminator: &Terminator,
    ) -> IntLocalId {
        match terminator {
            Terminator::Exit(exit) => match body.exit(*exit) {
                FunctionExit::Return(value) => *value,
                FunctionExit::TailCall { .. } => panic!("fixture should return an Int local"),
            },
            _ => panic!("fixture should return an Int local"),
        }
    }

    fn returned_exit(exit: &FunctionExit<IntLocalId, usize>) -> IntLocalId {
        match exit {
            FunctionExit::Return(value) => *value,
            FunctionExit::TailCall { .. } => panic!("fixture should contain a return exit"),
        }
    }

    fn tail_call_exit(exit: &FunctionExit<IntLocalId, usize>) -> (usize, &[ParamLocal]) {
        match exit {
            FunctionExit::TailCall { function, args } => (*function, args),
            FunctionExit::Return(_) => panic!("fixture should contain a tail-call exit"),
        }
    }

    fn assert_int_shape(plan: &ExecutionPlan, shape: execution::type_::ValueShapeId) {
        assert_eq!(
            plan.shape_value_type(shape),
            execution::type_::ValueType::Int
        );
    }

    fn lowering_context() -> super::super::super::LoweringContext {
        super::super::super::LoweringContext::new(
            HashMap::new(),
            RepresentationContext::new(Vec::new()),
            super::super::super::ProgramConstantTemplates {
                modules: Vec::new(),
            },
            SpecializationKey::monomorphic(crate::plan::FunctionTemplateId::new(0)),
            HashSet::new(),
        )
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
