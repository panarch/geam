use super::super::super::{ExecutionPlan, ParamLocal, ParamSlot};
use super::local::ExplainLocal;
use super::{write_list, write_type};

pub(in crate::plan::execution::explain) fn write_slot(
    output: &mut String,
    plan: &ExecutionPlan,
    slot: &ParamSlot,
) {
    slot.local().write_local(output);
    output.push(':');
    output.push_str("shape#");
    output.push_str(&slot.shape().index().to_string());
    output.push('(');
    write_type(output, &plan.shape_value_type(slot.shape()));
    output.push(')');
}

pub(in crate::plan::execution::explain) fn write_slots(
    output: &mut String,
    plan: &ExecutionPlan,
    slots: &[ParamSlot],
) {
    write_list(output, slots, |output, slot| write_slot(output, plan, slot));
}

pub(in crate::plan::execution::explain) fn write_locals(
    output: &mut String,
    locals: &[ParamLocal],
) {
    write_list(output, locals, |output, local| local.write_local(output));
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, ParamLocal};

    #[test]
    fn writes_slots_and_local_argument_packs() {
        let source = "pub fn main() { 1 }";
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let instruction =
            &plan.int_function(IntFunctionId(0)).graph().blocks()[0].instructions()[0];
        let mut output = String::new();

        super::write_slot(&mut output, &plan, instruction.output());
        assert_eq!(output, "%int#0:shape#0(Int)");

        output.clear();
        super::write_locals(
            &mut output,
            &[ParamLocal::Int(crate::plan::execution::IntLocalId(2))],
        );
        assert_eq!(output, "[%int#2]");
    }
}
