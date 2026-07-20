use super::function::ExecutableFunction;
use super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionReturn, CustomListFunctionId, CustomReturn,
    ExecutionPlan, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionId,
    FunctionFunctionId, FunctionFunctionReturn, FunctionListFunctionId, GenericFunctionFunctionId,
    IntFunctionFunctionId, IntFunctionId, IntListFunctionId, ListFunctionFunctionId,
    ListFunctionId, ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId,
    NilFunctionFunctionId, NilFunctionId, NilListFunctionId, ParameterListFunctionId,
    ParameterListListFunctionId, ReturnBlock, ReturnGraph, ReturnTailCallId, ReturnTarget,
    RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId, StringListFunctionId,
    TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId, TypedFunctionReturn,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use std::fmt;

pub struct ExecutionPlanExplanation<'a> {
    plan: &'a ExecutionPlan,
}

impl<'a> ExecutionPlanExplanation<'a> {
    pub(super) fn new(plan: &'a ExecutionPlan) -> Self {
        Self { plan }
    }
}

impl fmt::Display for ExecutionPlanExplanation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&render(self.plan))
    }
}

fn render(plan: &ExecutionPlan) -> String {
    let mut output = String::new();
    output.push_str("module ");
    output.push_str(&plan.module);
    output.push_str("\nmain ");
    runtime_function_label(&plan.main).push_to(&mut output);
    output.push('\n');

    let functions = &plan.functions;
    write_table(&mut output, "never", &functions.never_functions);
    write_table(&mut output, "int", &functions.int_functions);
    write_table(&mut output, "float", &functions.float_functions);
    write_table(&mut output, "string", &functions.string_functions);
    write_table(&mut output, "bit_array", &functions.bit_array_functions);
    write_table(
        &mut output,
        "utf_codepoint",
        &functions.utf_codepoint_functions,
    );
    write_table(&mut output, "custom", &functions.custom_functions);
    write_table(&mut output, "bool", &functions.bool_functions);
    write_table(&mut output, "nil", &functions.nil_functions);
    write_table(&mut output, "tuple", &functions.tuple_functions);

    write_table(
        &mut output,
        "list.parameter",
        functions
            .parameter_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.int",
        functions
            .int_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.string",
        functions
            .string_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.bit_array",
        functions
            .bit_array_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.utf_codepoint",
        functions
            .utf_codepoint_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.custom",
        functions
            .custom_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.float",
        functions
            .float_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.bool",
        functions
            .bool_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.nil",
        functions
            .nil_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.tuple",
        functions
            .tuple_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.parameter_list",
        functions
            .parameter_list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.list",
        functions
            .list_list_functions
            .iter()
            .map(|(_, function)| function),
    );
    write_table(
        &mut output,
        "list.function",
        functions
            .function_list_functions
            .iter()
            .map(|(_, function)| function),
    );

    write_table(
        &mut output,
        "function.int",
        &functions.int_function_functions,
    );
    write_table(
        &mut output,
        "function.float",
        &functions.float_function_functions,
    );
    write_table(
        &mut output,
        "function.string",
        &functions.string_function_functions,
    );
    write_table(
        &mut output,
        "function.bit_array",
        &functions.bit_array_function_functions,
    );
    write_table(
        &mut output,
        "function.utf_codepoint",
        &functions.utf_codepoint_function_functions,
    );
    write_table(
        &mut output,
        "function.custom",
        &functions.custom_function_functions,
    );
    write_table(
        &mut output,
        "function.bool",
        &functions.bool_function_functions,
    );
    write_table(
        &mut output,
        "function.nil",
        &functions.nil_function_functions,
    );
    write_table(
        &mut output,
        "function.tuple",
        &functions.tuple_function_functions,
    );
    write_table(
        &mut output,
        "function.generic",
        &functions.generic_function_functions,
    );
    write_table(
        &mut output,
        "function.never",
        &functions.never_function_functions,
    );

    write_table(
        &mut output,
        "function.list.parameter",
        &functions.parameter_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.parameter_list",
        &functions.parameter_list_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.int",
        &functions.int_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.string",
        &functions.string_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.bit_array",
        &functions.bit_array_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.utf_codepoint",
        &functions.utf_codepoint_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.custom",
        &functions.custom_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.float",
        &functions.float_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.bool",
        &functions.bool_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.nil",
        &functions.nil_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.tuple",
        &functions.tuple_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.list",
        &functions.list_list_function_functions,
    );
    write_table(
        &mut output,
        "function.list.function",
        &functions.function_list_function_functions,
    );
    write_table(
        &mut output,
        "function.function",
        &functions.function_function_functions,
    );

    output
}

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

trait ExplainedReturn {
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

struct ExplainedTailCall {
    function_index: usize,
    argument_count: usize,
}

fn write_table<'a, Return, Functions>(
    output: &mut String,
    family: &'static str,
    functions: Functions,
) where
    Return: ExplainedReturn + 'a,
    Functions: IntoIterator<Item = &'a ExecutableFunction<Return>>,
{
    for (index, function) in functions.into_iter().enumerate() {
        output.push_str("\nfunction ");
        FunctionLabel::new(family, index).push_to(output);
        output.push_str("\n  entry steps=");
        output.push_str(&function.steps().len().to_string());
        output.push('\n');
        write_graph(output, function.return_(), family);
    }
}

fn write_graph(output: &mut String, graph: &dyn ExplainedReturn, family: &'static str) {
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

#[derive(Clone, Copy)]
struct FunctionLabel {
    family: &'static str,
    index: usize,
}

impl FunctionLabel {
    fn new(family: &'static str, index: usize) -> Self {
        Self { family, index }
    }

    fn push_to(self, output: &mut String) {
        output.push_str(self.family);
        output.push('#');
        output.push_str(&self.index.to_string());
    }
}

fn runtime_function_label(function: &RuntimeFunctionId) -> FunctionLabel {
    match function {
        RuntimeFunctionId::Never(id) => FunctionLabel::new("never", id.0),
        RuntimeFunctionId::Int(id) => FunctionLabel::new("int", id.0),
        RuntimeFunctionId::Float(id) => FunctionLabel::new("float", id.0),
        RuntimeFunctionId::String(id) => FunctionLabel::new("string", id.0),
        RuntimeFunctionId::BitArray(id) => FunctionLabel::new("bit_array", id.0),
        RuntimeFunctionId::UtfCodepoint(id) => FunctionLabel::new("utf_codepoint", id.0),
        RuntimeFunctionId::Custom(id) => FunctionLabel::new("custom", id.index()),
        RuntimeFunctionId::Bool(id) => FunctionLabel::new("bool", id.0),
        RuntimeFunctionId::Nil(id) => FunctionLabel::new("nil", id.0),
        RuntimeFunctionId::Tuple { id, .. } => FunctionLabel::new("tuple", id.0),
        RuntimeFunctionId::List(id) => list_function_label(id),
        RuntimeFunctionId::Function { id, .. } => function_function_label(id),
    }
}

fn list_function_label(function: &ListFunctionId) -> FunctionLabel {
    match function {
        ListFunctionId::Parameter(id) => FunctionLabel::new("list.parameter", id.index()),
        ListFunctionId::ParameterList(id) => FunctionLabel::new("list.parameter_list", id.index()),
        ListFunctionId::Int(id) => FunctionLabel::new("list.int", id.index()),
        ListFunctionId::String(id) => FunctionLabel::new("list.string", id.index()),
        ListFunctionId::BitArray(id) => FunctionLabel::new("list.bit_array", id.index()),
        ListFunctionId::UtfCodepoint(id) => FunctionLabel::new("list.utf_codepoint", id.index()),
        ListFunctionId::Custom(id) => FunctionLabel::new("list.custom", id.index()),
        ListFunctionId::Float(id) => FunctionLabel::new("list.float", id.index()),
        ListFunctionId::Bool(id) => FunctionLabel::new("list.bool", id.index()),
        ListFunctionId::Nil(id) => FunctionLabel::new("list.nil", id.index()),
        ListFunctionId::Tuple(id) => FunctionLabel::new("list.tuple", id.index()),
        ListFunctionId::List(id) => FunctionLabel::new("list.list", id.index()),
        ListFunctionId::Function(id) => FunctionLabel::new("list.function", id.index()),
    }
}

fn function_function_label(function: &FunctionFunctionId) -> FunctionLabel {
    match function {
        FunctionFunctionId::Generic(id) => FunctionLabel::new("function.generic", id.index()),
        FunctionFunctionId::Never(id) => FunctionLabel::new("function.never", id.index()),
        FunctionFunctionId::Int(id) => FunctionLabel::new("function.int", id.0),
        FunctionFunctionId::Float(id) => FunctionLabel::new("function.float", id.0),
        FunctionFunctionId::String(id) => FunctionLabel::new("function.string", id.0),
        FunctionFunctionId::BitArray(id) => FunctionLabel::new("function.bit_array", id.0),
        FunctionFunctionId::UtfCodepoint(id) => FunctionLabel::new("function.utf_codepoint", id.0),
        FunctionFunctionId::Custom(id) => FunctionLabel::new("function.custom", id.index()),
        FunctionFunctionId::Bool(id) => FunctionLabel::new("function.bool", id.0),
        FunctionFunctionId::Nil(id) => FunctionLabel::new("function.nil", id.0),
        FunctionFunctionId::Tuple(id) => FunctionLabel::new("function.tuple", id.0),
        FunctionFunctionId::List(id) => list_function_function_label(id),
        FunctionFunctionId::Function(id) => FunctionLabel::new("function.function", id.index()),
    }
}

fn list_function_function_label(function: &ListFunctionFunctionId) -> FunctionLabel {
    match function {
        ListFunctionFunctionId::Parameter { id, .. } => {
            FunctionLabel::new("function.list.parameter", id.0)
        }
        ListFunctionFunctionId::ParameterList { id, .. } => {
            FunctionLabel::new("function.list.parameter_list", id.0)
        }
        ListFunctionFunctionId::Int { id, .. } => FunctionLabel::new("function.list.int", id.0),
        ListFunctionFunctionId::String { id, .. } => {
            FunctionLabel::new("function.list.string", id.0)
        }
        ListFunctionFunctionId::BitArray { id, .. } => {
            FunctionLabel::new("function.list.bit_array", id.0)
        }
        ListFunctionFunctionId::UtfCodepoint { id, .. } => {
            FunctionLabel::new("function.list.utf_codepoint", id.0)
        }
        ListFunctionFunctionId::Custom { id, .. } => {
            FunctionLabel::new("function.list.custom", id.0)
        }
        ListFunctionFunctionId::Float { id, .. } => FunctionLabel::new("function.list.float", id.0),
        ListFunctionFunctionId::Bool { id, .. } => FunctionLabel::new("function.list.bool", id.0),
        ListFunctionFunctionId::Nil { id, .. } => FunctionLabel::new("function.list.nil", id.0),
        ListFunctionFunctionId::Tuple { id, .. } => FunctionLabel::new("function.list.tuple", id.0),
        ListFunctionFunctionId::List { id, .. } => FunctionLabel::new("function.list.list", id.0),
        ListFunctionFunctionId::Function { id, .. } => {
            FunctionLabel::new("function.list.function", id.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ExecutionPlan, ExecutionPlanExplanation, Value, run_main};

    #[test]
    fn explain_formats_public_return_graph_without_source_names_and_preserves_execution() {
        let source = include_str!("../../../tests/fixtures/explain/return_topology.gleam");
        let plan = execution_plan(source);
        assert_eq!(run_main(&plan), Ok(Value::Int(40.into())));

        let explanation: ExecutionPlanExplanation<'_> = plan.explain();
        assert_eq!(explanation.to_string(), expected_explanation(source));
        assert!(!explanation.to_string().contains("choose"));
        assert_eq!(run_main(&plan), Ok(Value::Int(40.into())));
    }

    #[test]
    fn explain_formats_exact_never_block() {
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

    #[test]
    fn explain_formats_every_function_table_group_in_storage_order() {
        let fixtures = [
            include_str!("../../../tests/fixtures/explain/value_return_tables.gleam"),
            include_str!("../../../tests/fixtures/explain/list_return_tables.gleam"),
            include_str!("../../../tests/fixtures/explain/function_return_tables.gleam"),
            include_str!("../../../tests/fixtures/explain/list_returning_function_tables.gleam"),
            include_str!("../../../tests/fixtures/explain/return_table_group_order.gleam"),
        ];

        for source in fixtures {
            assert_eq!(
                execution_plan(source).explain().to_string(),
                expected_explanation(source),
            );
        }
    }

    fn expected_explanation(source: &str) -> String {
        let (_, comments) = source
            .split_once("\n// geam:explain\n")
            .expect("explain fixture should contain an expected output block");
        let mut expected = String::new();

        for line in comments.lines() {
            let comment = line
                .strip_prefix("//")
                .expect("expected output lines should be comments");
            expected.push_str(comment.strip_prefix(' ').unwrap_or(comment));
            expected.push('\n');
        }

        expected
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
