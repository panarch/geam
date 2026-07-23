mod instruction;
mod terminator;

pub(crate) use instruction::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment, BoolInstruction,
    CustomInstruction, FloatInstruction, FunctionCapture, FunctionInstruction,
    FunctionInstructionKind, FunctionTarget, Instruction, InstructionKind, IntInstruction,
    ListInstruction, NilInstruction, ParameterListInstruction, StringInstruction, TupleInstruction,
    TypedListInstruction, UtfCodepointInstruction,
};
pub(crate) use terminator::{
    BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternSizeExpr, BitArrayPatternValue, BitArrayStringPattern, BoolBranch, Edge,
    FloatSwitch, IntSwitch, Jump, LetAssertPanic, Match, MatchEdge, MatchEdgeArgument,
    MatchIntBindingId, MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail,
    NeverCall, NeverCallTarget, Signedness, SourceStop, SourceStopKind, StringSwitch, Terminator,
};

use crate::plan::execution::ParamSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BlockId(usize);

pub(crate) struct Block {
    params: Box<[ParamSlot]>,
    instructions: Box<[Instruction]>,
    terminator: Terminator,
}

impl BlockId {
    pub(in crate::plan::execution) fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl Block {
    pub(in crate::plan::execution) fn new(
        params: Vec<ParamSlot>,
        instructions: Vec<Instruction>,
        terminator: Terminator,
    ) -> Self {
        Self {
            params: params.into_boxed_slice(),
            instructions: instructions.into_boxed_slice(),
            terminator,
        }
    }

    pub(crate) fn params(&self) -> &[ParamSlot] {
        &self.params
    }

    pub(crate) fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub(crate) fn terminator(&self) -> &Terminator {
        &self.terminator
    }
}

use crate::plan::execution::explain::ExplainContext;
use crate::plan::execution::graph::GraphExitId;

pub(super) fn write_block(
    context: &mut ExplainContext<'_, '_>,
    index: usize,
    block: &Block,
    write_exit: &mut dyn FnMut(&mut ExplainContext<'_, '_>, GraphExitId),
) {
    context.push_str("  block b");
    context.push_str(&index.to_string());
    context.push_str(" params=");
    context.write_list(block.params(), |context, slot| context.write(slot));
    context.push('\n');
    for instruction in block.instructions() {
        context.write(instruction);
    }
    context.push_str("    ");
    terminator::write_terminator(context, block.terminator(), write_exit);
    context.push('\n');
}

#[cfg(test)]
mod explain_tests {
    use super::write_block;
    use crate::plan::execution::{IntFunctionId, explain};

    #[test]
    fn writes_block_parameters_instructions_and_terminator() {
        let source = "pub fn main() { 1 }";
        let expected = concat!(
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    exit#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let block = &plan.int_function(IntFunctionId(0)).graph().blocks()[0];
            let mut context = explain::ExplainContext::new(plan, output);
            write_block(&mut context, 0, block, &mut |context, exit| {
                context.push_str("exit#");
                context.push_str(&exit.index().to_string());
            });
        });
    }
}
