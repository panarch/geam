use super::super::super::ExecutionPlan;
use super::super::super::graph::Block;
use super::super::instruction::write_instruction;
use super::super::value::write_slots;

pub(super) fn write_block(output: &mut String, plan: &ExecutionPlan, index: usize, block: &Block) {
    output.push_str("  block b");
    output.push_str(&index.to_string());
    output.push_str(" params=");
    write_slots(output, plan, block.params());
    output.push('\n');
    for instruction in block.instructions() {
        write_instruction(output, plan, instruction);
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::IntFunctionId;

    #[test]
    fn writes_block_parameters_and_instructions() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let block = &plan.int_function(IntFunctionId(0)).graph().blocks()[0];
        let mut output = String::new();

        super::write_block(&mut output, &plan, 0, block);

        assert_eq!(
            output,
            concat!(
                "  block b0 params=[]\n",
                "    %int#0:shape#0(Int) = int.value 1\n",
            ),
        );
    }
}
