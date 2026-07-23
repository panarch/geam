use super::super::constant::{ConstantTable, ConstantValue};
use super::super::graph::FunctionLocal;
use super::super::{
    BitArrayListLocalId, BitArrayLocalId, BoolListLocalId, BoolLocalId, CustomListLocalId,
    CustomLocal, ExecutionPlan, FloatListLocalId, FloatLocalId, FunctionListLocalId,
    IntListLocalId, IntLocalId, ListListLocalId, NilListLocalId, NilLocalId,
    ParameterListListLocalId, ParameterListLocalId, StringListLocalId, StringLocalId,
    TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
};
use super::graph::write_constant_program;

pub(super) fn write_constant_tables(
    output: &mut String,
    plan: &ExecutionPlan,
    constants: &ConstantTable,
) {
    write_table::<IntLocalId>(output, plan, constants, "int");
    write_table::<FloatLocalId>(output, plan, constants, "float");
    write_table::<StringLocalId>(output, plan, constants, "string");
    write_table::<BitArrayLocalId>(output, plan, constants, "bit_array");
    write_table::<CustomLocal>(output, plan, constants, "custom");
    write_table::<BoolLocalId>(output, plan, constants, "bool");
    write_table::<NilLocalId>(output, plan, constants, "nil");
    write_table::<TupleLocalId>(output, plan, constants, "tuple");
    write_table::<ParameterListLocalId>(output, plan, constants, "list.parameter");
    write_table::<ParameterListListLocalId>(output, plan, constants, "list.parameter_list");
    write_table::<IntListLocalId>(output, plan, constants, "list.int");
    write_table::<StringListLocalId>(output, plan, constants, "list.string");
    write_table::<BitArrayListLocalId>(output, plan, constants, "list.bit_array");
    write_table::<UtfCodepointListLocalId>(output, plan, constants, "list.utf_codepoint");
    write_table::<CustomListLocalId>(output, plan, constants, "list.custom");
    write_table::<FloatListLocalId>(output, plan, constants, "list.float");
    write_table::<BoolListLocalId>(output, plan, constants, "list.bool");
    write_table::<NilListLocalId>(output, plan, constants, "list.nil");
    write_table::<TupleListLocalId>(output, plan, constants, "list.tuple");
    write_table::<ListListLocalId>(output, plan, constants, "list.list");
    write_table::<FunctionListLocalId>(output, plan, constants, "list.function");
    write_table::<FunctionLocal>(output, plan, constants, "function");
}

fn write_table<Value>(
    output: &mut String,
    plan: &ExecutionPlan,
    constants: &ConstantTable,
    family: &'static str,
) where
    Value: ConstantValue + super::value::ExplainLocal,
{
    for (index, program) in Value::programs(constants).iter().enumerate() {
        output.push_str("\nconstant.");
        output.push_str(family);
        output.push('#');
        output.push_str(&index.to_string());
        output.push('\n');
        write_constant_program(output, plan, program);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn writes_constant_programs_in_family_order() {
        let source = r#"
const enabled = True
const one = 1
pub fn main() { #(one, enabled) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let mut output = String::new();

        super::write_constant_tables(&mut output, &plan, &plan.constants);

        assert_eq!(
            output,
            concat!(
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
            ),
        );
    }
}
