use super::{ConstantId, ProfiledConstantProgram};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionLabelSource, HostedExecutionGraph,
};
use crate::plan::execution::graph::FunctionLocal;
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    BitArrayListLocalId, BitArrayLocalId, BoolListLocalId, BoolLocalId, CustomListLocalId,
    CustomLocal, ExternalListLocalId, FloatListLocalId, FloatLocalId, FunctionListLocalId,
    IntListLocalId, IntLocalId, ListListLocalId, NilListLocalId, NilLocalId,
    ParameterListListLocalId, ParameterListLocalId, StringListLocalId, StringLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
};

pub(crate) struct ProfiledConstantTable<Graph: ExecutionGraphProfile> {
    ints: Vec<ProfiledConstantProgram<IntLocalId, Graph>>,
    strings: Vec<ProfiledConstantProgram<StringLocalId, Graph>>,
    bit_arrays: Vec<ProfiledConstantProgram<BitArrayLocalId, Graph>>,
    customs: Vec<ProfiledConstantProgram<CustomLocal, Graph>>,
    floats: Vec<ProfiledConstantProgram<FloatLocalId, Graph>>,
    bools: Vec<ProfiledConstantProgram<BoolLocalId, Graph>>,
    nils: Vec<ProfiledConstantProgram<NilLocalId, Graph>>,
    tuples: Vec<ProfiledConstantProgram<TupleLocalId, Graph>>,
    parameter_lists: Vec<ProfiledConstantProgram<ParameterListLocalId, Graph>>,
    parameter_list_lists: Vec<ProfiledConstantProgram<ParameterListListLocalId, Graph>>,
    int_lists: Vec<ProfiledConstantProgram<IntListLocalId, Graph>>,
    string_lists: Vec<ProfiledConstantProgram<StringListLocalId, Graph>>,
    bit_array_lists: Vec<ProfiledConstantProgram<BitArrayListLocalId, Graph>>,
    utf_codepoint_lists: Vec<ProfiledConstantProgram<UtfCodepointListLocalId, Graph>>,
    custom_lists: Vec<ProfiledConstantProgram<CustomListLocalId, Graph>>,
    external_lists: Vec<ProfiledConstantProgram<ExternalListLocalId, Graph>>,
    float_lists: Vec<ProfiledConstantProgram<FloatListLocalId, Graph>>,
    bool_lists: Vec<ProfiledConstantProgram<BoolListLocalId, Graph>>,
    nil_lists: Vec<ProfiledConstantProgram<NilListLocalId, Graph>>,
    tuple_lists: Vec<ProfiledConstantProgram<TupleListLocalId, Graph>>,
    list_lists: Vec<ProfiledConstantProgram<ListListLocalId, Graph>>,
    function_lists: Vec<ProfiledConstantProgram<FunctionListLocalId, Graph>>,
    functions: Vec<ProfiledConstantProgram<FunctionLocal, Graph>>,
}

impl<Graph: ExecutionGraphProfile> Default for ProfiledConstantTable<Graph> {
    fn default() -> Self {
        Self {
            ints: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            customs: Vec::new(),
            floats: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            tuples: Vec::new(),
            parameter_lists: Vec::new(),
            parameter_list_lists: Vec::new(),
            int_lists: Vec::new(),
            string_lists: Vec::new(),
            bit_array_lists: Vec::new(),
            utf_codepoint_lists: Vec::new(),
            custom_lists: Vec::new(),
            external_lists: Vec::new(),
            float_lists: Vec::new(),
            bool_lists: Vec::new(),
            nil_lists: Vec::new(),
            tuple_lists: Vec::new(),
            list_lists: Vec::new(),
            function_lists: Vec::new(),
            functions: Vec::new(),
        }
    }
}

pub(crate) type ConstantTable = ProfiledConstantTable<HostedExecutionGraph>;

pub(crate) trait ConstantValue: Sized {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>];
    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>>;
}

impl ConstantValue for IntLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.ints
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.ints
    }
}

impl ConstantValue for StringLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.strings
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.strings
    }
}

impl ConstantValue for BitArrayLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bit_arrays
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.bit_arrays
    }
}

impl ConstantValue for CustomLocal {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.customs
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.customs
    }
}

impl ConstantValue for FloatLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.floats
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.floats
    }
}

impl ConstantValue for BoolLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bools
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.bools
    }
}

impl ConstantValue for NilLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.nils
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.nils
    }
}

impl ConstantValue for TupleLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.tuples
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.tuples
    }
}

impl ConstantValue for ParameterListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.parameter_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.parameter_lists
    }
}

impl ConstantValue for ParameterListListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.parameter_list_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.parameter_list_lists
    }
}

impl ConstantValue for IntListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.int_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.int_lists
    }
}

impl ConstantValue for StringListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.string_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.string_lists
    }
}

impl ConstantValue for BitArrayListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bit_array_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.bit_array_lists
    }
}

impl ConstantValue for UtfCodepointListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.utf_codepoint_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.utf_codepoint_lists
    }
}

impl ConstantValue for CustomListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.custom_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.custom_lists
    }
}

impl ConstantValue for ExternalListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.external_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.external_lists
    }
}

impl ConstantValue for FloatListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.float_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.float_lists
    }
}

impl ConstantValue for BoolListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bool_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.bool_lists
    }
}

impl ConstantValue for NilListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.nil_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.nil_lists
    }
}

impl ConstantValue for TupleListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.tuple_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.tuple_lists
    }
}

impl ConstantValue for ListListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.list_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.list_lists
    }
}

impl ConstantValue for FunctionListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.function_lists
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.function_lists
    }
}

impl ConstantValue for FunctionLocal {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.functions
    }

    fn programs_mut<Graph: ExecutionGraphProfile>(
        table: &mut ProfiledConstantTable<Graph>,
    ) -> &mut Vec<ProfiledConstantProgram<Self, Graph>> {
        &mut table.functions
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledConstantTable<Graph> {
    pub(in crate::plan::execution) fn push<Return: ConstantValue>(
        &mut self,
        program: ProfiledConstantProgram<Return, Graph>,
    ) -> ConstantId<Return> {
        let programs = Return::programs_mut(self);
        let id = ConstantId::new(programs.len());
        programs.push(program);
        id
    }

    pub(crate) fn get<Return: ConstantValue>(
        &self,
        id: ConstantId<Return>,
    ) -> &ProfiledConstantProgram<Return, Graph> {
        &Return::programs(self)[id.index()]
    }
}

impl<Graph> Explain for ProfiledConstantTable<Graph>
where
    Graph: ExecutionGraphProfile,
    Graph::ExternalFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionId: FunctionLabelSource,
    Graph::ExternalFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalInstruction: Explain,
    Graph::ExternalListInstruction: Explain,
    Graph::ExternalFunctionInstruction: Explain,
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_table::<IntLocalId, _>(context, self, "int");
        write_table::<FloatLocalId, _>(context, self, "float");
        write_table::<StringLocalId, _>(context, self, "string");
        write_table::<BitArrayLocalId, _>(context, self, "bit_array");
        write_table::<CustomLocal, _>(context, self, "custom");
        write_table::<BoolLocalId, _>(context, self, "bool");
        write_table::<NilLocalId, _>(context, self, "nil");
        write_table::<TupleLocalId, _>(context, self, "tuple");
        write_table::<ParameterListLocalId, _>(context, self, "list.parameter");
        write_table::<ParameterListListLocalId, _>(context, self, "list.parameter_list");
        write_table::<IntListLocalId, _>(context, self, "list.int");
        write_table::<StringListLocalId, _>(context, self, "list.string");
        write_table::<BitArrayListLocalId, _>(context, self, "list.bit_array");
        write_table::<UtfCodepointListLocalId, _>(context, self, "list.utf_codepoint");
        write_table::<CustomListLocalId, _>(context, self, "list.custom");
        write_table::<ExternalListLocalId, _>(context, self, "list.external");
        write_table::<FloatListLocalId, _>(context, self, "list.float");
        write_table::<BoolListLocalId, _>(context, self, "list.bool");
        write_table::<NilListLocalId, _>(context, self, "list.nil");
        write_table::<TupleListLocalId, _>(context, self, "list.tuple");
        write_table::<ListListLocalId, _>(context, self, "list.list");
        write_table::<FunctionListLocalId, _>(context, self, "list.function");
        write_table::<FunctionLocal, _>(context, self, "function");
    }
}

fn write_table<Value, Graph>(
    context: &mut ExplainContext<'_, '_>,
    constants: &ProfiledConstantTable<Graph>,
    family: &'static str,
) where
    Value: ConstantValue + LocalLabel,
    Graph: ExecutionGraphProfile,
    Graph::ExternalFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionId: FunctionLabelSource,
    Graph::ExternalFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalListFunctionFunctionId: FunctionLabelSource,
    Graph::ExternalInstruction: Explain,
    Graph::ExternalListInstruction: Explain,
    Graph::ExternalFunctionInstruction: Explain,
{
    for (index, program) in Value::programs(constants).iter().enumerate() {
        context.push_str("\nconstant.");
        context.push_str(family);
        context.push('#');
        context.push_str(&index.to_string());
        context.push('\n');
        context.write(program);
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;

    #[test]
    fn writes_constant_programs_in_family_order() {
        let source = r#"
const enabled = True
const one = 1
pub fn main() { #(one, enabled) }
"#;
        let expected = concat!(
            "\nconstant.int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
            "\nconstant.bool#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %bool#0:shape#1(Bool) = bool.value True\n",
            "    return %bool#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&plan.program.common.constants);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantId;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::function::ExecutionGraphProfile;
    use crate::plan::execution::graph::{
        BlockId, IntLocalId, ParamLocal, ProfiledInstruction, ProfiledInstructionKind, Terminator,
    };
    use crate::plan::execution::graph::{IntInstruction, SourceStop, SourceStopKind};
    use num_bigint::BigInt;
    #[test]
    fn constant_entry_is_a_reusable_zero_argument_typed_graph_program() {
        let plan = execution_plan("const one = 1 pub fn main() { one + one }");
        let program = plan.constant(ConstantId::<IntLocalId>::new(0));
        let block_graph = program.block_graph();

        assert_eq!(block_graph.entry(), BlockId::new(0));
        assert_eq!(block_graph.blocks().len(), 1);
        let block = block_graph.block(BlockId::new(0));
        assert!(block.params().is_empty());
        assert_eq!(block.instructions().len(), 1);
        let instruction = &block.instructions()[0];
        assert_eq!(
            instruction.output().local(),
            &ParamLocal::Int(IntLocalId(0))
        );
        assert_eq!(int_literal(instruction), &1.into());
        assert_eq!(returned_int(program, block.terminator()), IntLocalId(0));

        let main = plan.int_function(crate::plan::execution::function::IntFunctionId(0));
        let block = main.body().block_graph().block(BlockId::new(0));
        assert_eq!(block.instructions().len(), 3);
        for (index, instruction) in block.instructions()[..2].iter().enumerate() {
            let output = IntLocalId(index);
            assert_eq!(instruction.output().local(), &ParamLocal::Int(output));
            assert_eq!(int_constant(instruction), ConstantId::new(0));
        }
    }

    #[test]
    #[should_panic(expected = "constant fixture should contain an Int literal")]
    fn int_literal_guard_rejects_other_instructions() {
        let plan = execution_plan("const one = 1 pub fn main() { one }");
        let graph = plan
            .int_function(crate::plan::execution::function::IntFunctionId(0))
            .body()
            .block_graph();
        int_literal(&graph.block(graph.entry()).instructions()[0]);
    }

    #[test]
    #[should_panic(expected = "constant fixture should return an Int local")]
    fn returned_int_guard_rejects_other_terminators() {
        let plan = execution_plan("const one = 1 pub fn main() { one }");
        let program = plan.constant(ConstantId::<IntLocalId>::new(0));
        returned_int(
            program,
            &Terminator::SourceStop(SourceStop::new(
                SourceStopKind::Panic,
                None,
                crate::plan::PanicSite::unknown(),
            )),
        );
    }

    #[test]
    #[should_panic(expected = "constant fixture should reference an Int constant")]
    fn int_constant_guard_rejects_other_instructions() {
        let plan = execution_plan("const one = 1 pub fn main() { one }");
        let graph = plan
            .constant(ConstantId::<IntLocalId>::new(0))
            .block_graph();
        int_constant(&graph.block(graph.entry()).instructions()[0]);
    }

    fn int_literal<Graph: ExecutionGraphProfile>(
        instruction: &ProfiledInstruction<Graph>,
    ) -> &BigInt {
        match instruction.kind() {
            ProfiledInstructionKind::Int(IntInstruction::Value(value)) => value,
            _ => panic!("constant fixture should contain an Int literal"),
        }
    }

    fn returned_int<Graph: ExecutionGraphProfile>(
        program: &crate::plan::execution::constant::ProfiledConstantProgram<IntLocalId, Graph>,
        terminator: &Terminator,
    ) -> IntLocalId {
        match terminator {
            Terminator::Exit(exit) => *program.return_(*exit),
            _ => panic!("constant fixture should return an Int local"),
        }
    }

    fn int_constant<Graph: ExecutionGraphProfile>(
        instruction: &ProfiledInstruction<Graph>,
    ) -> ConstantId<IntLocalId> {
        match instruction.kind() {
            ProfiledInstructionKind::Int(IntInstruction::Constant(id)) => *id,
            _ => panic!("constant fixture should reference an Int constant"),
        }
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
