use super::super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionReturn, CustomListFunctionId, CustomReturn,
    FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
    IntListFunctionId, ListFunctionFunctionId, ListListFunctionId, NeverFunctionFunctionId,
    NeverFunctionId, NilFunctionFunctionId, NilFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, ReturnBlock, ReturnGraph,
    ReturnTailCallId, ReturnTarget, StringFunctionFunctionId, StringFunctionId,
    StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId,
    TypedFunctionReturn, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionId,
};
use super::label::FunctionLabel;

trait ReturnFunctionIndex {
    fn return_function_index(&self) -> usize;
}

impl ReturnFunctionIndex for usize {
    fn return_function_index(&self) -> usize {
        *self
    }
}

impl ReturnFunctionIndex for NeverFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for IntFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for FloatFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for StringFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for BitArrayFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for UtfCodepointFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for BoolFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for NilFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for TupleFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for ParameterListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for IntListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for StringListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for BitArrayListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for UtfCodepointListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for CustomListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for FloatListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for BoolListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for NilListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for TupleListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for ParameterListListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for ListListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for FunctionListFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for IntFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for FloatFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for StringFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for BitArrayFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for UtfCodepointFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for GenericFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for NeverFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.index()
    }
}

impl ReturnFunctionIndex for BoolFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for NilFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for TupleFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        self.0
    }
}

impl ReturnFunctionIndex for ListFunctionFunctionId {
    fn return_function_index(&self) -> usize {
        match self {
            Self::Parameter { id, .. } => id.0,
            Self::ParameterList { id, .. } => id.0,
            Self::Int { id, .. } => id.0,
            Self::String { id, .. } => id.0,
            Self::BitArray { id, .. } => id.0,
            Self::UtfCodepoint { id, .. } => id.0,
            Self::Custom { id, .. } => id.0,
            Self::Float { id, .. } => id.0,
            Self::Bool { id, .. } => id.0,
            Self::Nil { id, .. } => id.0,
            Self::Tuple { id, .. } => id.0,
            Self::List { id, .. } => id.0,
            Self::Function { id, .. } => id.0,
        }
    }
}

pub(super) trait ExplainedReturn {
    fn entry(&self) -> ReturnTarget;
    fn blocks(&self) -> &[ReturnBlock];
    fn tail_call(&self, id: ReturnTailCallId) -> ExplainedTailCall;
}

impl<Expression, Function> ExplainedReturn for ReturnGraph<Expression, Function>
where
    Function: ReturnFunctionIndex,
{
    fn entry(&self) -> ReturnTarget {
        self.entry()
    }

    fn blocks(&self) -> &[ReturnBlock] {
        self.blocks()
    }

    fn tail_call(&self, id: ReturnTailCallId) -> ExplainedTailCall {
        let call = self.tail_call(id);
        ExplainedTailCall {
            function_index: call.function().return_function_index(),
            argument_count: call.args().len(),
        }
    }
}

impl<Expression, Function> ExplainedReturn
    for TypedFunctionReturn<ReturnGraph<Expression, Function>>
where
    Function: ReturnFunctionIndex,
{
    fn entry(&self) -> ReturnTarget {
        self.body().entry()
    }

    fn blocks(&self) -> &[ReturnBlock] {
        self.body().blocks()
    }

    fn tail_call(&self, id: ReturnTailCallId) -> ExplainedTailCall {
        ExplainedReturn::tail_call(self.body(), id)
    }
}

impl ExplainedReturn for CustomReturn {
    fn entry(&self) -> ReturnTarget {
        self.body().entry()
    }

    fn blocks(&self) -> &[ReturnBlock] {
        self.body().blocks()
    }

    fn tail_call(&self, id: ReturnTailCallId) -> ExplainedTailCall {
        ExplainedReturn::tail_call(self.body(), id)
    }
}

impl ExplainedReturn for CustomFunctionReturn {
    fn entry(&self) -> ReturnTarget {
        self.body().entry()
    }

    fn blocks(&self) -> &[ReturnBlock] {
        self.body().blocks()
    }

    fn tail_call(&self, id: ReturnTailCallId) -> ExplainedTailCall {
        ExplainedReturn::tail_call(self.body(), id)
    }
}

impl ExplainedReturn for FunctionFunctionReturn {
    fn entry(&self) -> ReturnTarget {
        self.body().entry()
    }

    fn blocks(&self) -> &[ReturnBlock] {
        self.body().blocks()
    }

    fn tail_call(&self, id: ReturnTailCallId) -> ExplainedTailCall {
        ExplainedReturn::tail_call(self.body(), id)
    }
}

pub(super) struct ExplainedTailCall {
    function_index: usize,
    argument_count: usize,
}

pub(super) fn write_graph(output: &mut String, graph: &dyn ExplainedReturn, family: &'static str) {
    output.push_str("  graph entry=b");
    output.push_str(&graph.entry().index().to_string());
    output.push('\n');
    for (index, block) in graph.blocks().iter().enumerate() {
        output.push_str("  b");
        output.push_str(&index.to_string());
        output.push(' ');
        match block {
            ReturnBlock::Return { .. } => output.push_str("return\n"),
            ReturnBlock::Never(_) => output.push_str("never\n"),
            ReturnBlock::TailCall { call } => {
                let call = graph.tail_call(*call);
                output.push_str("tail ");
                FunctionLabel::new(family, call.function_index).push_to(output);
                output.push_str(" args=");
                output.push_str(&call.argument_count.to_string());
                output.push('\n');
            }
            ReturnBlock::BoolBranch { true_, false_, .. } => {
                output.push_str("branch bool true=b");
                output.push_str(&true_.index().to_string());
                output.push_str(" false=b");
                output.push_str(&false_.index().to_string());
                output.push('\n');
            }
            ReturnBlock::IntSwitch {
                clauses, fallback, ..
            } => {
                output.push_str("switch int");
                for (pattern, target) in clauses {
                    output.push(' ');
                    output.push_str(&pattern.to_string());
                    output.push_str("->b");
                    output.push_str(&target.index().to_string());
                }
                output.push_str(" fallback=b");
                output.push_str(&fallback.index().to_string());
                output.push('\n');
            }
            ReturnBlock::FloatSwitch {
                clauses, fallback, ..
            } => {
                output.push_str("switch float");
                for (pattern, target) in clauses {
                    output.push(' ');
                    output.push_str(&format!("{pattern:?}"));
                    output.push_str("->b");
                    output.push_str(&target.index().to_string());
                }
                output.push_str(" fallback=b");
                output.push_str(&fallback.index().to_string());
                output.push('\n');
            }
            ReturnBlock::StringSwitch {
                clauses, fallback, ..
            } => {
                output.push_str("switch string");
                for (pattern, target) in clauses {
                    output.push(' ');
                    output.push_str(&format!("{pattern:?}"));
                    output.push_str("->b");
                    output.push_str(&target.index().to_string());
                }
                output.push_str(" fallback=b");
                output.push_str(&fallback.index().to_string());
                output.push('\n');
            }
            ReturnBlock::Steps { steps, next } => {
                output.push_str("steps count=");
                output.push_str(&steps.len().to_string());
                output.push_str(" next=b");
                output.push_str(&next.index().to_string());
                output.push('\n');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ExecutionPlan;

    #[test]
    fn formats_exact_never_block() {
        let plan = execution_plan(
            r#"
fn stop() -> value { panic as "stop" }
fn consume(value: value) -> Int { consume(value) }

pub fn main() { consume(stop()) }
"#,
        );

        assert_eq!(
            plan.explain().to_string(),
            concat!(
                "module main\n",
                "main int#0\n",
                "\n",
                "function never#0\n",
                "  entry steps=0\n",
                "  graph entry=b0\n",
                "  b0 return\n",
                "\n",
                "function int#0\n",
                "  entry steps=0\n",
                "  graph entry=b0\n",
                "  b0 never\n",
            ),
        );
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
