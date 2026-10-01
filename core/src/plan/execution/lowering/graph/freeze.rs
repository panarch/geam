mod alignment;
pub(in crate::plan::execution::lowering::graph) mod arithmetic;
mod instruction;
mod integer;
mod pattern;
mod transfer;
mod value;

use super::draft::{
    DraftBlock, DraftBlockId, DraftEdge, DraftGraph, DraftGraphBuilder, DraftGraphValue,
    DraftMatchEdge, DraftMatchEdgeArgument, DraftNeverCallTarget, DraftTailCall, DraftTerminator,
    DraftValueRef, LoweredFunctionGraph,
};
use super::liveness::GraphLiveness;
use crate::plan::execution;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use value::BlockValues;
pub(in crate::plan::execution::lowering) use value::FreezeGraphValue;

struct BlockLayout {
    parameters: Vec<ParameterSource>,
    id: execution::graph::BlockId,
    params: Vec<execution::graph::ParamSlot>,
    values: BlockValues,
}

#[derive(Clone, PartialEq, Eq)]
enum ParameterSource {
    Explicit(usize),
    Inherited(DraftValueRef),
}

impl ParameterSource {
    fn value<'a>(&'a self, block: &'a DraftBlock) -> &'a DraftValueRef {
        match self {
            Self::Explicit(index) => &block.explicit_params[*index],
            Self::Inherited(value) => value,
        }
    }
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
        transfer: execution::graph::Transfer,
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
                FrozenGraphExit::TailCall {
                    function,
                    args,
                    transfer,
                } => execution::function::FunctionExit::TailCall {
                    function,
                    args: args.into(),
                    transfer,
                },
            })
            .collect();
        execution::function::FunctionBody::from_parts(frozen.graph, exits)
    })
}

pub(super) fn freeze_constant<Return>(
    graph: DraftGraphBuilder<Return, Infallible>,
    shape: &super::super::specialization::SpecializedValueShape,
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
    let shape = context.types.value_shape(shape);
    execution::constant::ConstantProgram::from_parts(frozen.graph, returns, shape)
}

fn freeze_graph<Return, TailCall>(
    mut graph: DraftGraphBuilder<Return, TailCall>,
    context: &mut super::super::LoweringContext,
) -> LoweredFunctionGraph<FrozenGraph<Return::Frozen, TailCall>>
where
    Return: DraftGraphValue + FreezeGraphValue,
    TailCall: Clone,
{
    integer::use_immediates(&mut graph);
    arithmetic::form_regions(&mut graph);
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
    let mut layouts = draft_blocks
        .iter()
        .map(|(id, block)| {
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
            (*id, block_layout(block_ids[id], block, parameters, context))
        })
        .collect::<HashMap<_, _>>();
    alignment::select(&draft_blocks, entry, &mut layouts, context);
    let mut exits = Vec::new();
    let mut blocks = Vec::with_capacity(draft_blocks.len());
    for (draft_id, block) in draft_blocks {
        let DraftBlock {
            explicit_params: _,
            instructions,
            terminator,
        } = block;
        let layout = &layouts[&draft_id];
        let instructions = instructions
            .iter()
            .map(|draft| instruction::freeze(draft, &layout.values, context))
            .collect();
        let terminator = freeze_terminator(
            terminator,
            &returns,
            &tail_calls,
            layouts[&entry].id,
            layout,
            &layouts,
            &mut exits,
        );
        blocks.push(execution::graph::Block::new(
            layout.params.clone(),
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
    id: execution::graph::BlockId,
    block: &DraftBlock,
    parameters: Vec<ParameterSource>,
    context: &mut super::super::LoweringContext,
) -> BlockLayout {
    let mut values = BlockValues::default();
    let params = parameters
        .iter()
        .map(|source| values.allocate(source.value(block), context))
        .collect();
    for instruction in &block.instructions {
        for output in instruction.outputs() {
            values.allocate(&output, context);
        }
    }
    BlockLayout {
        id,
        parameters,
        params,
        values,
    }
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
    tail_calls: &[DraftTailCall<TailCall>],
    entry: execution::graph::BlockId,
    layout: &BlockLayout,
    layouts: &HashMap<DraftBlockId, BlockLayout>,
    exits: &mut Vec<FrozenGraphExit<Return::Frozen, TailCall>>,
) -> execution::graph::Terminator
where
    Return: DraftGraphValue + FreezeGraphValue,
    TailCall: Clone,
{
    use execution::graph::Terminator as E;

    match terminator {
        DraftTerminator::Jump(edge) => E::Jump(execution::graph::Jump::new(freeze_edge(
            &edge, layout, layouts,
        ))),
        DraftTerminator::BoolBranch {
            subject,
            true_,
            false_,
        } => E::BoolBranch(execution::graph::BoolBranch::new(
            layout.values.bool(&subject),
            freeze_edge(&true_, layout, layouts),
            freeze_edge(&false_, layout, layouts),
        )),
        DraftTerminator::IntSwitch {
            subject,
            clauses,
            fallback,
        } => E::IntSwitch(execution::graph::IntSwitch::new(
            layout.values.int(&subject),
            clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern.into(), freeze_edge(&edge, layout, layouts)))
                .collect::<Vec<_>>()
                .into(),
            freeze_edge(&fallback, layout, layouts),
        )),
        DraftTerminator::FloatSwitch {
            subject,
            clauses,
            fallback,
        } => E::FloatSwitch(execution::graph::FloatSwitch::new(
            layout.values.float(&subject),
            clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern, freeze_edge(&edge, layout, layouts)))
                .collect::<Vec<_>>()
                .into(),
            freeze_edge(&fallback, layout, layouts),
        )),
        DraftTerminator::StringSwitch {
            subject,
            clauses,
            fallback,
        } => E::StringSwitch(execution::graph::StringSwitch::new(
            layout.values.string(&subject),
            clauses
                .into_iter()
                .map(|(pattern, edge)| (pattern.into(), freeze_edge(&edge, layout, layouts)))
                .collect::<Vec<_>>()
                .into(),
            freeze_edge(&fallback, layout, layouts),
        )),
        DraftTerminator::Match {
            subject,
            pattern: draft_pattern,
            success,
            failure,
        } => E::Match(execution::graph::Match::new(
            layout.values.any(&subject),
            pattern::freeze(draft_pattern, &layout.values),
            freeze_match_edge(&success, layout, &layouts[&success.target]),
            freeze_edge(&failure, layout, layouts),
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
            freeze_edge(&next, layout, layouts),
        )),
        DraftTerminator::Return { value: _, index } => {
            let id = execution::graph::BlockGraphExitId::new(exits.len());
            exits.push(FrozenGraphExit::Return(
                returns[index].freeze(&layout.values),
            ));
            E::Exit(id)
        }
        DraftTerminator::TailCall { function, args } => {
            let args = layout.values.any_slice(&args);
            let transfer = transfer::arguments(layout, &args);
            match &tail_calls[function] {
                DraftTailCall::Entry => E::Jump(execution::graph::Jump::new(
                    execution::graph::Edge::new(entry, args.into_vec(), transfer),
                )),
                DraftTailCall::Function(function) => {
                    let id = execution::graph::BlockGraphExitId::new(exits.len());
                    exits.push(FrozenGraphExit::TailCall {
                        function: function.clone(),
                        args,
                        transfer,
                    });
                    E::Exit(id)
                }
            }
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
        } => {
            let args = layout.values.any_slice(&args);
            let transfer = transfer::arguments(layout, &args);
            E::NeverCall(execution::graph::NeverCall::new(
                match function {
                    DraftNeverCallTarget::Direct(function) => {
                        execution::graph::NeverCallTarget::Direct(function)
                    }
                    DraftNeverCallTarget::Value(function) => {
                        execution::graph::NeverCallTarget::Value(
                            layout.values.never_function(&function),
                        )
                    }
                },
                args.into(),
                transfer,
                site,
            ))
        }
    }
}

fn freeze_edge(
    edge: &DraftEdge,
    source: &BlockLayout,
    layouts: &HashMap<DraftBlockId, BlockLayout>,
) -> execution::graph::Edge {
    let target = &layouts[&edge.target];
    let args = edge_arguments(edge, source, target);
    let transfer = transfer::arguments(source, &args);
    execution::graph::Edge::new(target.id, args, transfer)
}

fn edge_arguments(
    edge: &DraftEdge,
    source: &BlockLayout,
    target: &BlockLayout,
) -> Vec<execution::graph::ParamLocal> {
    target
        .parameters
        .iter()
        .map(|parameter| {
            let value = match parameter {
                ParameterSource::Explicit(index) => &edge.explicit_args[*index],
                ParameterSource::Inherited(value) => value,
            };
            source.values.any(value)
        })
        .collect()
}

fn freeze_match_edge(
    edge: &DraftMatchEdge,
    source: &BlockLayout,
    target: &BlockLayout,
) -> execution::graph::MatchEdge {
    let args = match_arguments(edge, source, target);
    let (bindings, transfer) = transfer::matched(source, &args, &target.params);
    execution::graph::MatchEdge::new(target.id, args, bindings, transfer)
}

fn match_arguments(
    edge: &DraftMatchEdge,
    source: &BlockLayout,
    target: &BlockLayout,
) -> Vec<execution::graph::MatchEdgeArgument> {
    use execution::graph::MatchEdgeArgument;
    target
        .parameters
        .iter()
        .map(|parameter| match parameter {
            ParameterSource::Explicit(index) => match &edge.explicit_args[*index] {
                DraftMatchEdgeArgument::Binding(index) => MatchEdgeArgument::Binding(*index),
            },
            ParameterSource::Inherited(value) => MatchEdgeArgument::Value(source.values.any(value)),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::super::draft::instruction::{
        DraftBoolInstruction, DraftIntInstruction, DraftIntegerOperand,
    };
    use super::super::draft::{DraftGraphBuilder, DraftInt, DraftTailCall};
    use super::freeze;
    use crate::Value;
    use crate::plan::FunctionCallTarget;
    use crate::plan::execution;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::function::{
        ExecutionGraphProfile, FunctionExit, IntFunctionId, ProfiledFunctionBody,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, BoolLocalId, Edge, IntInstruction, IntLocalId, IntegerOperand,
        NilLocalId, ParamLocal, ProfiledInstruction, ProfiledInstructionKind, StorageFamily,
        Terminator, Transfer,
    };
    use crate::plan::execution::lowering::specialization::{
        RepresentationContext, SpecializationKey, StoredValueShape,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::storage::Table;
    use crate::runtime::run_main;
    use std::collections::{HashMap, HashSet};
    use std::convert::Infallible;

    type FunctionBody<Return, TailCall> = ProfiledFunctionBody<Return, TailCall, Infallible>;

    #[derive(Clone, Copy)]
    enum IntBinaryOperation {
        Add,
        Multiply,
    }

    #[test]
    fn self_tail_uses_the_entry_layout_and_swap_transfer_without_an_exit() {
        let plan = execution_plan(
            r#"
fn spin(left: Int, right: Int, marker: Nil) -> Int {
  spin(right, left, marker)
}

pub fn main() { spin(1, 2, Nil) }
"#,
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let graph = body.block_graph();
        assert_eq!(graph.blocks().len(), 1);
        let edge = jump(graph.block(graph.entry()).terminator());
        assert_eq!(edge.target(), graph.entry());
        assert_eq!(
            edge.args(),
            &[
                ParamLocal::Int(IntLocalId(1)),
                ParamLocal::Int(IntLocalId(0)),
                ParamLocal::Nil(NilLocalId(0)),
            ],
        );
        assert_eq!(
            edge.transfer
                .families
                .iter()
                .map(|family| (
                    family.family,
                    family.length,
                    family
                        .steps
                        .iter()
                        .map(|step| (step.source, step.destination))
                        .collect::<Vec<_>>(),
                ))
                .collect::<Vec<_>>(),
            vec![(StorageFamily::Int, 2, vec![(1, 0)])],
        );
        assert!(body.exits.is_empty());

        let main = plan.int_function(IntFunctionId(0)).body();
        assert_eq!(main.exits.len(), 1);
        assert_eq!(
            exit_id(
                main.block_graph()
                    .block(main.block_graph().entry())
                    .terminator()
            ),
            BlockGraphExitId::new(0)
        );
    }

    #[test]
    fn generic_self_tails_keep_separate_int_and_string_specializations() {
        let plan = execution_plan(
            r#"
fn repeat(value: value, remaining: Int) -> value {
  case remaining {
    0 -> value
    n -> repeat(value, n - 1)
  }
}

pub fn main() { #(repeat(42, 2), repeat("retained", 3)) }
"#,
        );
        let int_body = plan.int_function(IntFunctionId(0)).body();
        let string_body = plan.program.functions.value_returns.string_functions[0].body();
        for graph in [int_body.block_graph(), string_body.block_graph()] {
            assert_eq!(graph.blocks().len(), 3);
            let edge = jump(graph.block(BlockId::new(2)).terminator());
            assert_eq!(edge.target(), graph.entry());
            assert_eq!(edge.args().len(), 2);
        }
        assert_eq!(int_body.exits.len(), 1);
        assert_eq!(string_body.exits.len(), 1);
        assert_eq!(
            run_main(&plan, &mut Vec::new()),
            Ok(Value::Tuple(vec![
                Value::Int(42.into()),
                Value::String("retained".into())
            ]))
        );
    }

    #[test]
    fn self_tail_arguments_evaluate_before_cyclic_and_duplicate_transfer() {
        let plan = execution_plan(
            r#"
fn rotate(left: Int, middle: Int, right: Int, remaining: Int) {
  case remaining {
    0 -> #(left, middle, right)
    _ -> rotate(echo middle, echo right, echo left, remaining - 1)
  }
}
fn duplicate(left: String, right: String, remaining: Int) {
  case remaining {
    0 -> #(left, right)
    _ -> duplicate(left, left, remaining - 1)
  }
}
pub fn main() { #(rotate(1, 2, 3, 2), duplicate("retained", "discarded", 3)) }
"#,
        );
        let mut echo = Vec::new();
        assert_eq!(
            run_main(&plan, &mut echo),
            Ok(Value::Tuple(vec![
                Value::Tuple(vec![
                    Value::Int(3.into()),
                    Value::Int(1.into()),
                    Value::Int(2.into())
                ]),
                Value::Tuple(vec![
                    Value::String("retained".into()),
                    Value::String("retained".into())
                ]),
            ])),
        );
        assert_eq!(
            echo.iter()
                .map(|output| output.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["2", "3", "1", "3", "1", "2"],
        );
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
        assert_eq!(merge.instructions().len(), 1);
        let multiply = &merge.instructions()[0];
        assert_eq!(
            multiply.value().unwrap().output().local(),
            &ParamLocal::Int(IntLocalId(1))
        );
        assert_int_shape(&plan, multiply.value().unwrap().output().shape());
        let (left, right) = int_binary_operands(multiply, IntBinaryOperation::Multiply);
        assert_eq!(
            (Rust::expression(&left), Rust::expression(&right)),
            (
                "data::graph::IntegerOperand::Local(data::graph::IntLocalId(0))".to_string(),
                "data::graph::IntegerOperand::Immediate(3)".to_string(),
            )
        );
        assert_eq!(returned_int(body, merge.terminator()), IntLocalId(1));

        assert_branch_add_and_jump(&plan, body, BlockId::new(3), 2, BlockId::new(2));
    }

    #[test]
    fn guarded_list_projection_is_emitted_only_in_the_body_that_uses_it() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let enabled = True
  case [42, 43] {
    [head, ..tail] if enabled -> head
    _ -> 0
  }
}
"#,
        );
        // The guard block b1 has no projections; b2 retains both original body bindings.
        let expected = concat!(
            "module main\n",
            "main int#0\n",
            "\n",
            "function int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %bool#0:shape#0(Bool) = bool.value True\n",
            "    %int#0:shape#1(Int) = int.value 42\n",
            "    %int#1:shape#1(Int) = int.value 43\n",
            "    %list.int#0:shape#2(list_type#0) = list.int[type#0] value elements=[%int#0, %int#1]\n",
            "    %bool#1:shape#0(Bool) = bool.list_length_at_least %list.int#0 length=1\n",
            "    branch %bool#1 true=b1(%bool#0, %list.int#0) false=b5()\n",
            "  block b1 params=[%bool#0:shape#0(Bool), %list.int#0:shape#2(list_type#0)]\n",
            "    branch %bool#0 true=b2(%list.int#0) false=b3()\n",
            "  block b2 params=[%list.int#0:shape#2(list_type#0)]\n",
            "    %int#0:shape#1(Int) = int.list_index %list.int#0 index=0\n",
            "    %list.int#1:shape#2(list_type#0) = list.int[type#0] drop_first %list.int#0 count=1\n",
            "    return %int#0\n",
            "  block b3 params=[]\n",
            "    jump b4()\n",
            "  block b4 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 0\n",
            "    return %int#0\n",
            "  block b5 params=[]\n",
            "    jump b4()\n",
        );
        assert_eq!(plan.explain().to_string(), expected);
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
                left: DraftIntegerOperand::Local(DraftInt::from_ref(&target_param)),
                right: DraftIntegerOperand::Local(inherited.clone()),
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
        let (left, right) = int_binary_operands(&target.instructions()[0], IntBinaryOperation::Add);
        assert_eq!(
            (Rust::expression(&left), Rust::expression(&right)),
            (
                "data::graph::IntegerOperand::Local(data::graph::IntLocalId(0))".to_string(),
                "data::graph::IntegerOperand::Local(data::graph::IntLocalId(1))".to_string(),
            )
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
        draft.finish_tail_call(
            tail_call_block,
            DraftTailCall::Function(7),
            vec![tail_arg.erase()],
        );
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
            Edge::new(
                BlockId::new(0),
                Vec::new(),
                Transfer {
                    families: Table::Static(&[]),
                },
            ),
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
            args: Vec::new().into(),
            transfer: Transfer {
                families: Table::Static(&[]),
            },
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
        assert_eq!(block.instructions().len(), 1);
        let add = &block.instructions()[0];
        assert_eq!(
            add.value().unwrap().output().local(),
            &ParamLocal::Int(IntLocalId(1))
        );
        assert_int_shape(plan, add.value().unwrap().output().shape());
        assert!(matches!(
            int_binary_operands(add, IntBinaryOperation::Add),
            (IntegerOperand::Local(IntLocalId(0)), IntegerOperand::Immediate(value)) if value == addend
        ));

        let edge = jump(block.terminator());
        assert_eq!(edge.target(), target);
        assert_eq!(edge.args(), &[ParamLocal::Int(IntLocalId(1))]);
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
    ) -> (IntegerOperand, IntegerOperand) {
        match (operation, instruction.value().unwrap().kind()) {
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
            FunctionExit::TailCall { function, args, .. } => (*function, args),
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
