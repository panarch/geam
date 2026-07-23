use super::super::super::{ExecutionPlan, ParamSlot};
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

#[cfg(test)]
mod tests {
    use crate::plan::execution::IntFunctionId;

    #[test]
    fn writes_slot_from_a_lowered_instruction() {
        let source = "pub fn main() { 1 }";
        let expected = "%int#0:shape#0(Int)";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            let instruction =
                &plan.int_function(IntFunctionId(0)).graph().blocks()[0].instructions()[0];
            super::write_slot(output, plan, instruction.output());
        });
    }
}
