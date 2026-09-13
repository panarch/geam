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
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

pub struct ProfiledConstantTable<Graph: ExecutionGraphProfile> {
    pub ints: Table<ProfiledConstantProgram<IntLocalId, Graph>>,
    pub strings: Table<ProfiledConstantProgram<StringLocalId, Graph>>,
    pub bit_arrays: Table<ProfiledConstantProgram<BitArrayLocalId, Graph>>,
    pub customs: Table<ProfiledConstantProgram<CustomLocal, Graph>>,
    pub floats: Table<ProfiledConstantProgram<FloatLocalId, Graph>>,
    pub bools: Table<ProfiledConstantProgram<BoolLocalId, Graph>>,
    pub nils: Table<ProfiledConstantProgram<NilLocalId, Graph>>,
    pub tuples: Table<ProfiledConstantProgram<TupleLocalId, Graph>>,
    pub parameter_lists: Table<ProfiledConstantProgram<ParameterListLocalId, Graph>>,
    pub parameter_list_lists: Table<ProfiledConstantProgram<ParameterListListLocalId, Graph>>,
    pub int_lists: Table<ProfiledConstantProgram<IntListLocalId, Graph>>,
    pub string_lists: Table<ProfiledConstantProgram<StringListLocalId, Graph>>,
    pub bit_array_lists: Table<ProfiledConstantProgram<BitArrayListLocalId, Graph>>,
    pub utf_codepoint_lists: Table<ProfiledConstantProgram<UtfCodepointListLocalId, Graph>>,
    pub custom_lists: Table<ProfiledConstantProgram<CustomListLocalId, Graph>>,
    pub external_lists: Table<ProfiledConstantProgram<ExternalListLocalId, Graph>>,
    pub float_lists: Table<ProfiledConstantProgram<FloatListLocalId, Graph>>,
    pub bool_lists: Table<ProfiledConstantProgram<BoolListLocalId, Graph>>,
    pub nil_lists: Table<ProfiledConstantProgram<NilListLocalId, Graph>>,
    pub tuple_lists: Table<ProfiledConstantProgram<TupleListLocalId, Graph>>,
    pub list_lists: Table<ProfiledConstantProgram<ListListLocalId, Graph>>,
    pub function_lists: Table<ProfiledConstantProgram<FunctionListLocalId, Graph>>,
    pub functions: Table<ProfiledConstantProgram<FunctionLocal, Graph>>,
}

pub(crate) type ConstantTable = ProfiledConstantTable<HostedExecutionGraph>;

pub(crate) trait ConstantValue: Sized + 'static {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>];
}

impl ConstantValue for IntLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.ints
    }
}

impl ConstantValue for StringLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.strings
    }
}

impl ConstantValue for BitArrayLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bit_arrays
    }
}

impl ConstantValue for CustomLocal {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.customs
    }
}

impl ConstantValue for FloatLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.floats
    }
}

impl ConstantValue for BoolLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bools
    }
}

impl ConstantValue for NilLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.nils
    }
}

impl ConstantValue for TupleLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.tuples
    }
}

impl ConstantValue for ParameterListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.parameter_lists
    }
}

impl ConstantValue for ParameterListListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.parameter_list_lists
    }
}

impl ConstantValue for IntListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.int_lists
    }
}

impl ConstantValue for StringListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.string_lists
    }
}

impl ConstantValue for BitArrayListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bit_array_lists
    }
}

impl ConstantValue for UtfCodepointListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.utf_codepoint_lists
    }
}

impl ConstantValue for CustomListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.custom_lists
    }
}

impl ConstantValue for ExternalListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.external_lists
    }
}

impl ConstantValue for FloatListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.float_lists
    }
}

impl ConstantValue for BoolListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.bool_lists
    }
}

impl ConstantValue for NilListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.nil_lists
    }
}

impl ConstantValue for TupleListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.tuple_lists
    }
}

impl ConstantValue for ListListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.list_lists
    }
}

impl ConstantValue for FunctionListLocalId {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.function_lists
    }
}

impl ConstantValue for FunctionLocal {
    fn programs<Graph: ExecutionGraphProfile>(
        table: &ProfiledConstantTable<Graph>,
    ) -> &[ProfiledConstantProgram<Self, Graph>] {
        &table.functions
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledConstantTable<Graph> {
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

impl<Graph: ExecutionGraphProfile> Emit for ProfiledConstantTable<Graph>
where
    Table<ProfiledConstantProgram<IntLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<StringLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<BitArrayLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<CustomLocal, Graph>>: Emit,
    Table<ProfiledConstantProgram<FloatLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<BoolLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<NilLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<TupleLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<ParameterListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<ParameterListListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<IntListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<StringListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<BitArrayListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<UtfCodepointListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<CustomListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<ExternalListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<FloatListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<BoolListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<NilListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<TupleListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<ListListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<FunctionListLocalId, Graph>>: Emit,
    Table<ProfiledConstantProgram<FunctionLocal, Graph>>: Emit,
{
    fn emit(&self, output: &mut Rust) {
        let Self {
            ints,
            strings,
            bit_arrays,
            customs,
            floats,
            bools,
            nils,
            tuples,
            parameter_lists,
            parameter_list_lists,
            int_lists,
            string_lists,
            bit_array_lists,
            utf_codepoint_lists,
            custom_lists,
            external_lists,
            float_lists,
            bool_lists,
            nil_lists,
            tuple_lists,
            list_lists,
            function_lists,
            functions,
        } = self;
        output.structure(
            "constant::ProfiledConstantTable",
            &[
                ("ints", ints),
                ("strings", strings),
                ("bit_arrays", bit_arrays),
                ("customs", customs),
                ("floats", floats),
                ("bools", bools),
                ("nils", nils),
                ("tuples", tuples),
                ("parameter_lists", parameter_lists),
                ("parameter_list_lists", parameter_list_lists),
                ("int_lists", int_lists),
                ("string_lists", string_lists),
                ("bit_array_lists", bit_array_lists),
                ("utf_codepoint_lists", utf_codepoint_lists),
                ("custom_lists", custom_lists),
                ("external_lists", external_lists),
                ("float_lists", float_lists),
                ("bool_lists", bool_lists),
                ("nil_lists", nil_lists),
                ("tuple_lists", tuple_lists),
                ("list_lists", list_lists),
                ("function_lists", function_lists),
                ("functions", functions),
            ],
        );
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
            context.write(plan.program.common.constants.as_ref());
        });
    }
}
