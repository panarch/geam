mod bit_array;
mod block;
mod exit;
mod value;

pub(crate) use bit_array::{Endianness, FloatBitSize, StringEncoding};
pub(in crate::plan::execution::graph) use bit_array::{endianness, float_size, string_encoding};
pub(crate) use block::{
    BitArrayBindingPattern, BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction,
    BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize, BitArrayPatternSizeExpr,
    BitArrayPatternValue, BitArraySegment, BitArrayStringPattern, Block, BlockId, BoolBranch,
    BoolInstruction, CustomInstruction, Edge, FloatInstruction, FloatSwitch, FunctionCapture,
    FunctionInstruction, FunctionInstructionKind, FunctionTarget, Instruction, InstructionKind,
    IntInstruction, IntSwitch, Jump, LetAssertPanic, ListInstruction, Match, MatchEdge,
    MatchEdgeArgument, MatchIntBindingId, MatchPattern, MatchPatternBinding, MatchPatternList,
    MatchPatternListTail, NeverCall, NeverCallTarget, NilInstruction, ParameterListInstruction,
    Signedness, SourceStop, SourceStopKind, StringInstruction, StringSwitch, Terminator,
    TupleInstruction, TypedListInstruction, UtfCodepointInstruction,
};
pub(crate) use exit::GraphExitId;
pub(crate) use value::{
    BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
    BoolFunctionLocalId, BoolListFunctionLocalId, BoolListLocalId, BoolLocalId,
    CustomFunctionLocal, CustomFunctionLocalId, CustomListFunctionLocalId, CustomListLocalId,
    CustomLocal, CustomLocalId, FloatFunctionLocalId, FloatListFunctionLocalId, FloatListLocalId,
    FloatLocalId, FunctionFunctionLocal, FunctionFunctionLocalId, FunctionListFunctionLocalId,
    FunctionListLocalId, FunctionLocal, GenericFunctionLocal, GenericFunctionLocalId,
    IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListFunctionLocalId, ListListLocalId, ListLocal, NeverFunctionLocal, NeverFunctionLocalId,
    NilFunctionLocalId, NilListFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal, ParamSlot,
    ParameterListFunctionLocalId, ParameterListListFunctionLocalId, ParameterListListLocalId,
    ParameterListLocalId, StoredListLocal, StringFunctionLocalId, StringListFunctionLocalId,
    StringListLocalId, StringLocalId, TupleFunctionLocalId, TupleListFunctionLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListFunctionLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};
pub(in crate::plan::execution) use value::{ExplainLocal, write_locals};

use crate::plan::execution::explain::ExplainContext;

pub(crate) struct Graph {
    entry: BlockId,
    blocks: Box<[Block]>,
}

impl Graph {
    pub(in crate::plan::execution) fn from_parts(entry: BlockId, blocks: Vec<Block>) -> Self {
        Self {
            entry,
            blocks: blocks.into_boxed_slice(),
        }
    }

    pub(crate) fn entry(&self) -> BlockId {
        self.entry
    }

    pub(crate) fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub(crate) fn block(&self, id: BlockId) -> &Block {
        &self.blocks[id.index()]
    }
}

pub(in crate::plan::execution) fn write_graph(
    context: &mut ExplainContext<'_, '_>,
    graph: &Graph,
    entry_params: &[ParamSlot],
    entry_captures: &[ParamSlot],
    write_exit: &mut dyn FnMut(&mut ExplainContext<'_, '_>, GraphExitId),
) {
    context.push_str("  entry b");
    context.push_str(&graph.entry().index().to_string());
    context.push_str(" params=");
    context.write_list(entry_params, |context, slot| context.write(slot));
    context.push_str(" captures=");
    context.write_list(entry_captures, |context, slot| context.write(slot));
    context.push('\n');

    for (index, block) in graph.blocks().iter().enumerate() {
        block::write_block(context, index, block, write_exit);
    }
}

#[cfg(test)]
mod explain_tests {
    use super::write_graph;
    use crate::plan::execution::{IntFunctionId, explain};

    #[test]
    fn writes_complete_graph_entry_and_block_order() {
        let source = r#"
fn choose(flag: Bool) { case flag { True -> 1 False -> 0 } }
pub fn main() { choose(True) }
"#;
        let expected = concat!(
            "  entry b0 params=[%bool#0:shape#0(Bool)] captures=[]\n",
            "  block b0 params=[%bool#0:shape#0(Bool)]\n",
            "    branch %bool#0 true=b1() false=b2()\n",
            "  block b1 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 1\n",
            "    exit#0\n",
            "  block b2 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 0\n",
            "    exit#1\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(1));
            let body = function.graph();
            let mut context = explain::ExplainContext::new(plan, output);
            write_graph(
                &mut context,
                body.graph(),
                function.entry().params(body),
                function.entry().captures(body),
                &mut |context, exit| {
                    context.push_str("exit#");
                    context.push_str(&exit.index().to_string());
                },
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlockId, Edge, GraphExitId, Instruction, InstructionKind, IntInstruction, MatchEdge,
        MatchEdgeArgument, MatchPattern, MatchPatternList, Terminator,
    };
    use crate::plan::execution::{
        BoolLocalId, ExecutionPlan, FunctionGraph, FunctionGraphExit, IntFunctionId, IntLocalId,
        ListLocal, ParamLocal,
    };

    #[derive(Clone, Copy)]
    enum IntBinaryOperation {
        Add,
        Multiply,
    }

    #[test]
    fn lowered_graph_owns_dense_typed_values_merges_and_edge_arguments() {
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
        let graph = function.graph();

        assert_eq!(graph.entry(), BlockId::new(0));
        assert_eq!(graph.blocks().len(), 4);
        assert_eq!(
            function
                .entry()
                .params(graph)
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![
                &ParamLocal::Bool(BoolLocalId(0)),
                &ParamLocal::Int(IntLocalId(0)),
            ],
        );

        let entry = graph.block(BlockId::new(0));
        assert!(entry.instructions().is_empty());
        let (subject, true_, false_) = bool_branch(entry.terminator());
        assert_eq!(subject, BoolLocalId(0));
        assert_eq!(true_.target(), BlockId::new(1));
        assert_eq!(false_.target(), BlockId::new(3));
        assert_eq!(true_.args(), &[ParamLocal::Int(IntLocalId(0))]);
        assert_eq!(false_.args(), &[ParamLocal::Int(IntLocalId(0))]);

        assert_branch_add_and_jump(&plan, graph, BlockId::new(1), 1, BlockId::new(2));

        let merge = graph.block(BlockId::new(2));
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
        assert_eq!(returned_int(graph, merge.terminator()), IntLocalId(2));

        assert_branch_add_and_jump(&plan, graph, BlockId::new(3), 2, BlockId::new(2));
    }

    #[test]
    fn lowered_match_exports_bindings_only_through_the_success_edge() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let assert [first, second] = [1, 2]
  first + second
}
"#,
        );
        let graph = plan.int_function(IntFunctionId(0)).graph();

        assert_eq!(graph.blocks().len(), 3);
        let entry = graph.block(BlockId::new(0));
        let (pattern, success, failure) = match_terminator(entry.terminator());
        let list = list_pattern(pattern);
        assert_eq!(list.elements().len(), 2);
        assert_binding_pattern(&list.elements()[0]);
        assert_binding_pattern(&list.elements()[1]);
        assert!(list.tail().is_none());
        assert_eq!(success.target(), BlockId::new(1));
        assert_eq!(success.args().len(), 2);
        assert_eq!(binding_edge_argument(&success.args()[0]), 0);
        assert_eq!(binding_edge_argument(&success.args()[1]), 1);
        assert_eq!(failure.target(), BlockId::new(2));
        assert_eq!(failure.args().len(), 1);
        let failure_subject = list_local(&failure.args()[0]);

        let success_block = graph.block(success.target());
        assert_eq!(
            success_block
                .params()
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![
                &ParamLocal::Int(IntLocalId(0)),
                &ParamLocal::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(success_block.instructions().len(), 1);
        assert_eq!(
            int_binary_operands(&success_block.instructions()[0], IntBinaryOperation::Add),
            (IntLocalId(0), IntLocalId(1)),
        );

        let failure_block = graph.block(failure.target());
        assert_eq!(failure_block.params().len(), 1);
        assert_eq!(
            failure_block.params()[0].local(),
            &ParamLocal::List(failure_subject.clone()),
        );
        let (panic_subject, message) = let_assert_panic(failure_block.terminator());
        assert_eq!(panic_subject, failure_block.params()[0].local());
        assert_eq!(message, None);
    }

    #[test]
    fn lowered_match_does_not_thread_an_unused_binding() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let assert [first, second] = [1, 2]
  first
}
"#,
        );
        let graph = plan.int_function(IntFunctionId(0)).graph();
        let entry = graph.block(BlockId::new(0));
        let (pattern, success, _) = match_terminator(entry.terminator());
        let list = list_pattern(pattern);
        assert_eq!(list.elements().len(), 2);
        assert_binding_pattern(&list.elements()[0]);
        assert_binding_pattern(&list.elements()[1]);
        assert_eq!(success.args().len(), 1);
        assert_eq!(binding_edge_argument(&success.args()[0]), 0);

        let success_block = graph.block(success.target());
        assert_eq!(success_block.params().len(), 1);
        assert_eq!(
            success_block.params()[0].local(),
            &ParamLocal::Int(IntLocalId(0)),
        );
        assert_eq!(
            returned_int(graph, success_block.terminator()),
            IntLocalId(0)
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a Bool branch")]
    fn bool_branch_rejects_the_wrong_fixture_shape() {
        bool_branch(&Terminator::Exit(GraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a match terminator")]
    fn match_terminator_rejects_the_wrong_fixture_shape() {
        match_terminator(&Terminator::Exit(GraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a List pattern")]
    fn list_pattern_rejects_the_wrong_fixture_shape() {
        list_pattern(&MatchPattern::Discard);
    }

    #[test]
    #[should_panic(expected = "fixture should contain a binding pattern")]
    fn assert_binding_pattern_rejects_the_wrong_fixture_shape() {
        assert_binding_pattern(&MatchPattern::Discard);
    }

    #[test]
    #[should_panic(expected = "fixture should export a match binding")]
    fn binding_edge_argument_rejects_the_wrong_fixture_shape() {
        binding_edge_argument(&MatchEdgeArgument::Value(ParamLocal::Int(IntLocalId(0))));
    }

    #[test]
    #[should_panic(expected = "fixture should carry a List local")]
    fn list_local_rejects_the_wrong_fixture_shape() {
        list_local(&ParamLocal::Int(IntLocalId(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain the requested Int binary instruction")]
    fn int_binary_operands_rejects_the_wrong_fixture_shape() {
        let plan = execution_plan("pub fn main() { 1 }");
        let graph = plan.int_function(IntFunctionId(0)).graph();
        int_binary_operands(
            &graph.block(graph.entry()).instructions()[0],
            IntBinaryOperation::Add,
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain an Int value instruction")]
    fn int_value_rejects_the_wrong_fixture_shape() {
        let plan = execution_plan("pub fn main() { 1 + 2 }");
        let graph = plan.int_function(IntFunctionId(0)).graph();
        int_value(&graph.block(graph.entry()).instructions()[2]);
    }

    #[test]
    #[should_panic(expected = "fixture should return an Int local")]
    fn returned_int_rejects_the_wrong_fixture_shape() {
        let plan = execution_plan("pub fn main() { 1 }");
        let graph = plan.int_function(IntFunctionId(0)).graph();
        returned_int(
            graph,
            &Terminator::SourceStop(super::SourceStop::new(
                super::SourceStopKind::Panic,
                None,
                crate::plan::PanicSite::unknown(),
            )),
        );
    }

    #[test]
    #[should_panic(expected = "fixture should return an Int local")]
    fn returned_int_rejects_a_tail_call() {
        let plan = execution_plan(
            r#"
fn loop(value: Int) -> Int { loop(value) }
pub fn main() { loop(1) }
"#,
        );
        let graph = plan.int_function(IntFunctionId(0)).graph();
        returned_int(graph, graph.block(graph.entry()).terminator());
    }

    #[test]
    #[should_panic(expected = "fixture should contain a jump terminator")]
    fn jump_rejects_the_wrong_fixture_shape() {
        jump(&Terminator::Exit(GraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a let-assert panic")]
    fn let_assert_panic_rejects_the_wrong_fixture_shape() {
        let_assert_panic(&Terminator::Exit(GraphExitId::new(0)));
    }

    fn assert_branch_add_and_jump(
        plan: &ExecutionPlan,
        graph: &FunctionGraph<IntLocalId, IntFunctionId>,
        block_id: BlockId,
        addend: i64,
        target: BlockId,
    ) {
        let block = graph.block(block_id);
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

    fn assert_int_value(
        plan: &ExecutionPlan,
        instruction: &super::Instruction,
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

    fn match_terminator(terminator: &Terminator) -> (&MatchPattern, &MatchEdge, &Edge) {
        match terminator {
            Terminator::Match(matcher) => (matcher.pattern(), matcher.success(), matcher.failure()),
            _ => panic!("fixture should contain a match terminator"),
        }
    }

    fn list_pattern(pattern: &MatchPattern) -> &MatchPatternList {
        match pattern {
            MatchPattern::List(pattern) => pattern,
            _ => panic!("fixture should contain a List pattern"),
        }
    }

    fn assert_binding_pattern(pattern: &MatchPattern) {
        match pattern {
            MatchPattern::Bind(_) => {}
            _ => panic!("fixture should contain a binding pattern"),
        }
    }

    fn binding_edge_argument(argument: &MatchEdgeArgument) -> usize {
        match argument {
            MatchEdgeArgument::Binding(index) => *index,
            MatchEdgeArgument::Value(_) => {
                panic!("fixture should export a match binding")
            }
        }
    }

    fn list_local(local: &ParamLocal) -> &ListLocal {
        match local {
            ParamLocal::List(local) => local,
            _ => panic!("fixture should carry a List local"),
        }
    }

    fn int_binary_operands(
        instruction: &Instruction,
        operation: IntBinaryOperation,
    ) -> (IntLocalId, IntLocalId) {
        match (operation, instruction.kind()) {
            (
                IntBinaryOperation::Add,
                InstructionKind::Int(IntInstruction::Add { left, right }),
            )
            | (
                IntBinaryOperation::Multiply,
                InstructionKind::Int(IntInstruction::Mult { left, right }),
            ) => (*left, *right),
            _ => panic!("fixture should contain the requested Int binary instruction"),
        }
    }

    fn int_value(instruction: &Instruction) -> &num_bigint::BigInt {
        match instruction.kind() {
            InstructionKind::Int(IntInstruction::Value(value)) => value,
            _ => panic!("fixture should contain an Int value instruction"),
        }
    }

    fn returned_int(
        graph: &FunctionGraph<IntLocalId, IntFunctionId>,
        terminator: &Terminator,
    ) -> IntLocalId {
        match terminator {
            Terminator::Exit(exit) => match graph.exit(*exit) {
                FunctionGraphExit::Return(value) => *value,
                FunctionGraphExit::TailCall { .. } => {
                    panic!("fixture should return an Int local")
                }
            },
            _ => panic!("fixture should return an Int local"),
        }
    }

    fn jump(terminator: &Terminator) -> &Edge {
        match terminator {
            Terminator::Jump(jump) => jump.edge(),
            _ => panic!("fixture should contain a jump terminator"),
        }
    }

    fn let_assert_panic(
        terminator: &Terminator,
    ) -> (&ParamLocal, Option<crate::plan::execution::StringLocalId>) {
        match terminator {
            Terminator::LetAssertPanic(panic) => (panic.subject(), panic.message()),
            _ => panic!("fixture should contain a let-assert panic"),
        }
    }

    fn assert_int_shape(plan: &ExecutionPlan, shape: crate::plan::execution::ValueShapeId) {
        assert_eq!(
            plan.shape_value_type(shape),
            crate::plan::execution::ValueType::Int
        );
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
