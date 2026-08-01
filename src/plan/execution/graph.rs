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
    BoolInstruction, CustomInstruction, Echo, Edge, ExternalFunctionCallTarget,
    ExternalFunctionInstruction, ExternalFunctionInstructionKind, ExternalFunctionInstructionView,
    ExternalFunctionTarget, ExternalInstruction, ExternalInstructionRef, ExternalInstructionView,
    ExternalListInstruction, ExternalListInstructionView, FloatInstruction, FloatSwitch,
    FunctionCapture, FunctionInstruction, FunctionInstructionKind, FunctionTarget, Instruction,
    InstructionKind, IntInstruction, IntSwitch, Jump, LetAssertPanic, ListInstruction, Match,
    MatchEdge, MatchEdgeArgument, MatchIntBindingId, MatchPattern, MatchPatternBinding,
    MatchPatternList, MatchPatternListTail, NeverCall, NeverCallTarget, NilInstruction,
    ParameterListInstruction, ProfiledBlock, ProfiledInstruction, ProfiledInstructionKind,
    Signedness, SourceStop, SourceStopKind, StringInstruction, StringSwitch, Terminator,
    TupleInstruction, TypedListInstruction, UtfCodepointInstruction,
};
pub(crate) use exit::BlockGraphExitId;
pub(crate) use value::{
    BitArrayFunctionLocalId, BitArrayListFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
    BoolFunctionLocalId, BoolListFunctionLocalId, BoolListLocalId, BoolLocalId,
    CoreFunctionFunctionLocal, CoreFunctionFunctionLocalId, CustomFunctionLocal,
    CustomFunctionLocalId, CustomListFunctionLocalId, CustomListLocalId, CustomLocal,
    CustomLocalId, ExternalFunctionFunctionLocal, ExternalFunctionFunctionLocalId,
    ExternalFunctionLocal, ExternalFunctionLocalId, ExternalListFunctionLocalId,
    ExternalListLocalId, ExternalLocal, ExternalLocalId, FloatFunctionLocalId,
    FloatListFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionListFunctionLocalId, FunctionListLocalId, FunctionLocal, GenericFunctionLocal,
    GenericFunctionLocalId, IntFunctionLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListFunctionLocalId, ListListLocalId, ListLocal, NeverFunctionLocal,
    NeverFunctionLocalId, NilFunctionLocalId, NilListFunctionLocalId, NilListLocalId, NilLocalId,
    ParamLocal, ParamSlot, ParameterListFunctionLocalId, ParameterListListFunctionLocalId,
    ParameterListListLocalId, ParameterListLocalId, StoredListLocal, StringFunctionLocalId,
    StringListFunctionLocalId, StringListLocalId, StringLocalId, TupleFunctionLocalId,
    TupleListFunctionLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointListFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
};
pub(in crate::plan::execution) use value::{LocalLabel, write_local_labels};

use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};

pub(crate) struct ProfiledBlockGraph<Graph: ExecutionGraphProfile> {
    entry: BlockId,
    blocks: Box<[ProfiledBlock<Graph>]>,
}

pub(crate) type BlockGraph = ProfiledBlockGraph<HostedExecutionGraph>;

pub(in crate::plan::execution) trait BlockGraphExitExplanation {
    fn write_exit(&self, context: &mut ExplainContext<'_, '_>, exit: BlockGraphExitId);
}

pub(in crate::plan::execution::graph) struct BlockGraphExplainContext<'a, 'plan, 'output> {
    context: &'a mut ExplainContext<'plan, 'output>,
    exits: &'a dyn BlockGraphExitExplanation,
}

impl<Graph: ExecutionGraphProfile> ProfiledBlockGraph<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        entry: BlockId,
        blocks: Vec<ProfiledBlock<Graph>>,
    ) -> Self {
        Self {
            entry,
            blocks: blocks.into_boxed_slice(),
        }
    }

    pub(crate) fn entry(&self) -> BlockId {
        self.entry
    }

    pub(crate) fn blocks(&self) -> &[ProfiledBlock<Graph>] {
        &self.blocks
    }

    pub(crate) fn block(&self, id: BlockId) -> &ProfiledBlock<Graph> {
        &self.blocks[id.index()]
    }

    pub(in crate::plan::execution) fn into_parts(self) -> (BlockId, Box<[ProfiledBlock<Graph>]>) {
        (self.entry, self.blocks)
    }

    pub(in crate::plan::execution) fn write_explanation(
        &self,
        context: &mut ExplainContext<'_, '_>,
        entry_params: &[ParamSlot],
        entry_captures: &[ParamSlot],
        exits: &dyn BlockGraphExitExplanation,
    ) where
        Graph::ExternalFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionId: FunctionLabelSource,
        Graph::ExternalFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
        Graph::ExternalInstruction: Explain,
        Graph::ExternalListInstruction: Explain,
        Graph::ExternalFunctionInstruction: Explain,
    {
        context.push_str("  entry b");
        context.push_str(&self.entry().index().to_string());
        context.push_str(" params=");
        context.write_list(entry_params, |context, slot| context.write(slot));
        context.push_str(" captures=");
        context.write_list(entry_captures, |context, slot| context.write(slot));
        context.push('\n');

        let mut graph_context = BlockGraphExplainContext { context, exits };
        for (index, block) in self.blocks().iter().enumerate() {
            block.write_explanation(&mut graph_context, index);
        }
    }
}

impl BlockGraphExplainContext<'_, '_, '_> {
    pub(in crate::plan::execution::graph) fn push(&mut self, character: char) {
        self.context.push(character);
    }

    pub(in crate::plan::execution::graph) fn push_str(&mut self, text: &str) {
        self.context.push_str(text);
    }

    pub(in crate::plan::execution::graph) fn write<Value>(&mut self, value: &Value)
    where
        Value: Explain + ?Sized,
    {
        self.context.write(value);
    }

    pub(in crate::plan::execution::graph) fn write_list<Value>(
        &mut self,
        values: &[Value],
        mut write_value: impl FnMut(&mut Self, &Value),
    ) {
        self.push('[');
        for (index, value) in values.iter().enumerate() {
            if index > 0 {
                self.push_str(", ");
            }
            write_value(self, value);
        }
        self.push(']');
    }

    pub(in crate::plan::execution::graph) fn write_exit(&mut self, exit: BlockGraphExitId) {
        self.exits.write_exit(self.context, exit);
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

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
            "    return %int#0\n",
            "  block b2 params=[]\n",
            "    %int#0:shape#1(Int) = int.value 0\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_echo_terminator_through_block_graph() {
        let source = r#"
fn emit(value: Int) {
  echo value
}

pub fn main() {
  emit(1)
}
"#;
        let expected = concat!(
            "  entry b0 params=[%int#0:shape#0(Int)] captures=[]\n",
            "  block b0 params=[%int#0:shape#0(Int)]\n",
            "    echo subject=%int#0 message=none site=main::emit@25..35 next=b1(%int#0)\n",
            "  block b1 params=[%int#0:shape#0(Int)]\n",
            "    return %int#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(1));
            let body = function.body();
            let mut context = explain::ExplainContext::new(plan, output);
            body.write_explanation(
                &mut context,
                "int",
                function.entry().params(body),
                function.entry().captures(body),
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlockGraphExitId, BlockId, Edge, IntInstruction, MatchEdge, MatchEdgeArgument,
        MatchPattern, MatchPatternList, ProfiledInstruction, ProfiledInstructionKind, Terminator,
    };
    use crate::plan::FunctionCallTarget;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::function::{FunctionExit, IntFunctionId, ProfiledFunctionBody};
    use crate::plan::execution::graph::{BoolLocalId, IntLocalId, ListLocal, ParamLocal};
    use std::convert::Infallible;

    type Instruction = ProfiledInstruction<Infallible>;
    type InstructionKind = ProfiledInstructionKind<Infallible>;
    type FunctionBody<Return, TailCall> = ProfiledFunctionBody<Return, TailCall, Infallible>;

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
    fn lowered_match_exports_bindings_only_through_the_success_edge() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let assert [first, second] = [1, 2]
  first + second
}
"#,
        );
        let body = plan.int_function(IntFunctionId(0)).body();
        let block_graph = body.block_graph();

        assert_eq!(block_graph.blocks().len(), 3);
        let entry = block_graph.block(BlockId::new(0));
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

        let success_block = block_graph.block(success.target());
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

        let failure_block = block_graph.block(failure.target());
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
        let body = plan.int_function(IntFunctionId(0)).body();
        let block_graph = body.block_graph();
        let entry = block_graph.block(BlockId::new(0));
        let (pattern, success, _) = match_terminator(entry.terminator());
        let list = list_pattern(pattern);
        assert_eq!(list.elements().len(), 2);
        assert_binding_pattern(&list.elements()[0]);
        assert_binding_pattern(&list.elements()[1]);
        assert_eq!(success.args().len(), 1);
        assert_eq!(binding_edge_argument(&success.args()[0]), 0);

        let success_block = block_graph.block(success.target());
        assert_eq!(success_block.params().len(), 1);
        assert_eq!(
            success_block.params()[0].local(),
            &ParamLocal::Int(IntLocalId(0)),
        );
        assert_eq!(
            returned_int(body, success_block.terminator()),
            IntLocalId(0)
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a Bool branch")]
    fn bool_branch_rejects_the_wrong_fixture_shape() {
        bool_branch(&Terminator::Exit(BlockGraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a match terminator")]
    fn match_terminator_rejects_the_wrong_fixture_shape() {
        match_terminator(&Terminator::Exit(BlockGraphExitId::new(0)));
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
        let block_graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        int_binary_operands(
            &block_graph.block(block_graph.entry()).instructions()[0],
            IntBinaryOperation::Add,
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain an Int value instruction")]
    fn int_value_rejects_the_wrong_fixture_shape() {
        let plan = execution_plan("pub fn main() { 1 + 2 }");
        let block_graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        int_value(&block_graph.block(block_graph.entry()).instructions()[2]);
    }

    #[test]
    #[should_panic(expected = "fixture should return an Int local")]
    fn returned_int_rejects_the_wrong_fixture_shape() {
        let plan = execution_plan("pub fn main() { 1 }");
        let body = plan.int_function(IntFunctionId(0)).body();
        returned_int(
            body,
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
        let body = plan.int_function(IntFunctionId(0)).body();
        let block_graph = body.block_graph();
        returned_int(body, block_graph.block(block_graph.entry()).terminator());
    }

    #[test]
    #[should_panic(expected = "fixture should contain a jump terminator")]
    fn jump_rejects_the_wrong_fixture_shape() {
        jump(&Terminator::Exit(BlockGraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a let-assert panic")]
    fn let_assert_panic_rejects_the_wrong_fixture_shape() {
        let_assert_panic(&Terminator::Exit(BlockGraphExitId::new(0)));
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

    fn assert_int_value(
        plan: &ExecutionPlan,
        instruction: &Instruction,
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
        body: &FunctionBody<IntLocalId, FunctionCallTarget<IntFunctionId>>,
        terminator: &Terminator,
    ) -> IntLocalId {
        match terminator {
            Terminator::Exit(exit) => match body.exit(*exit) {
                FunctionExit::Return(value) => *value,
                FunctionExit::TailCall { .. } => {
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
    ) -> (
        &ParamLocal,
        Option<crate::plan::execution::graph::StringLocalId>,
    ) {
        match terminator {
            Terminator::LetAssertPanic(panic) => (panic.subject(), panic.message()),
            _ => panic!("fixture should contain a let-assert panic"),
        }
    }

    fn assert_int_shape(plan: &ExecutionPlan, shape: crate::plan::execution::type_::ValueShapeId) {
        assert_eq!(
            plan.shape_value_type(shape),
            crate::plan::execution::type_::ValueType::Int
        );
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
