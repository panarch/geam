use crate::plan::execution::graph::{ArithmeticNode, ArithmeticOperand, ArithmeticRegion};
use crate::runtime::graph::BlockEnvironment;
use crate::runtime::integer::IntegerValue;
use std::borrow::Cow;

/// One execution owns these bounded, reusable buffers across calls and yields.
/// Native regions never construct an IntegerValue for an internal node.
#[derive(Default)]
pub(in crate::runtime::graph) struct ArithmeticScratch {
    native: Vec<i128>,
    values: Vec<IntegerValue>,
    outputs: Vec<IntegerValue>,
}

impl ArithmeticScratch {
    pub(super) fn execute(
        &mut self,
        region: &ArithmeticRegion,
        environment: &mut BlockEnvironment,
    ) {
        self.native.clear();
        self.values.clear();
        self.outputs.clear();
        let native = region.native
            && region.inputs.iter().all(|input| {
                if let Some(value) = environment.int_ref(*input).small() {
                    self.native.push(i128::from(value));
                    true
                } else {
                    false
                }
            });
        if native {
            for node in &region.nodes {
                let operand = |operand| match operand {
                    ArithmeticOperand::Input(index) => self.native[index],
                    ArithmeticOperand::Value(index) => self.native[region.inputs.len() + index],
                    ArithmeticOperand::Immediate(value) => i128::from(value),
                };
                // Admission rechecks every intermediate's range. Division and
                // remainder by zero follow the existing Gleam integer contract.
                let value = match *node {
                    ArithmeticNode::Add(left, right) => operand(left) + operand(right),
                    ArithmeticNode::Subtract(left, right) => operand(left) - operand(right),
                    ArithmeticNode::Multiply(left, right) => operand(left) * operand(right),
                    ArithmeticNode::Divide(left, right) => {
                        let right = operand(right);
                        if right == 0 { 0 } else { operand(left) / right }
                    }
                    ArithmeticNode::Remainder(left, right) => {
                        let right = operand(right);
                        if right == 0 { 0 } else { operand(left) % right }
                    }
                    ArithmeticNode::Negate(value) => -operand(value),
                };
                self.native.push(value);
            }
            for output in &region.outputs {
                environment.push_int(self.native[region.inputs.len() + output.value].into());
            }
        } else {
            for node in &region.nodes {
                let operand = |operand| match operand {
                    ArithmeticOperand::Input(index) => {
                        Cow::Borrowed(environment.int_ref(region.inputs[index]))
                    }
                    ArithmeticOperand::Value(index) => Cow::Borrowed(&self.values[index]),
                    ArithmeticOperand::Immediate(value) => Cow::Owned(IntegerValue::from(value)),
                };
                let value = match *node {
                    ArithmeticNode::Add(left, right) => {
                        IntegerValue::add(&operand(left), &operand(right))
                    }
                    ArithmeticNode::Subtract(left, right) => {
                        IntegerValue::subtract(&operand(left), &operand(right))
                    }
                    ArithmeticNode::Multiply(left, right) => {
                        IntegerValue::multiply(&operand(left), &operand(right))
                    }
                    ArithmeticNode::Divide(left, right) => {
                        IntegerValue::divide(&operand(left), &operand(right))
                    }
                    ArithmeticNode::Remainder(left, right) => {
                        IntegerValue::remainder(&operand(left), &operand(right))
                    }
                    ArithmeticNode::Negate(value) => operand(value).negate(),
                };
                self.values.push(value);
            }
            // Outputs are strictly ordered, unique node IDs. Moving in reverse
            // order leaves every remaining output's original index unchanged.
            for output in region.outputs.iter().rev() {
                self.outputs.push(self.values.swap_remove(output.value));
            }
            for value in self.outputs.drain(..).rev() {
                environment.push_int(value);
            }
            self.values.clear();
        }
        self.native.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::ArithmeticScratch;
    use crate::plan::execution::graph::{
        ArithmeticNode as N, ArithmeticOperand as O, ArithmeticOutput, ArithmeticRegion,
        IntLocalId, ParamLocal, ParamSlot, native_proof,
    };
    use crate::plan::execution::type_::ValueShapeId;
    use crate::runtime::graph::{BlockEnvironment, RetainedValues};
    use num_bigint::BigInt;

    fn region(nodes: Vec<N>, outputs: &[usize]) -> ArithmeticRegion {
        ArithmeticRegion {
            native: native_proof(2, &nodes),
            inputs: vec![IntLocalId(0), IntLocalId(1)].into(),
            nodes: nodes.into(),
            outputs: outputs
                .iter()
                .enumerate()
                .map(|(index, value)| ArithmeticOutput {
                    value: *value,
                    slot: ParamSlot {
                        local: ParamLocal::Int(IntLocalId(index + 2)),
                        shape: ValueShapeId(0),
                    },
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }

    #[test]
    fn native_and_large_input_execution_preserve_outputs_and_reuse_bounded_storage() {
        let region = region(
            vec![
                N::Add(O::Input(0), O::Input(1)),
                N::Subtract(O::Value(0), O::Input(1)),
                N::Multiply(O::Value(1), O::Immediate(2)),
                N::Divide(O::Value(2), O::Immediate(2)),
                N::Remainder(O::Value(3), O::Immediate(7)),
                N::Negate(O::Value(4)),
            ],
            &[0, 1, 5],
        );
        assert!(region.native);
        let mut scratch = ArithmeticScratch::default();
        let huge: BigInt = BigInt::from(1_u8) << 256;
        for (left, right) in [
            (BigInt::from(i64::MAX), BigInt::from(1)),
            (huge.clone(), -huge),
            (i64::MIN.into(), (-1).into()),
            (9.into(), 0.into()),
        ] {
            let mut input = RetainedValues::empty();
            input.push_int(left.clone().into());
            input.push_int(right.clone().into());
            let mut environment = BlockEnvironment::from_retained(input);
            scratch.execute(&region, &mut environment);
            assert_eq!(environment.int_ref(IntLocalId(2)), &(left.clone() + &right));
            assert_eq!(environment.int_ref(IntLocalId(3)), &left);
            assert_eq!(environment.int_ref(IntLocalId(4)), &-(left % 7_u8));
            assert!(scratch.values.is_empty());
            assert!(scratch.outputs.is_empty());
            assert!(scratch.native.is_empty());
        }
        assert!(scratch.native.capacity() >= 8);
        assert!(scratch.values.capacity() >= 6);
    }

    #[test]
    fn generic_unproved_regions_promote_demote_and_handle_zero_divisors() {
        let generic = region(
            vec![
                N::Multiply(O::Input(0), O::Input(0)),
                N::Multiply(O::Value(0), O::Value(0)),
                N::Divide(O::Value(1), O::Value(0)),
                N::Divide(O::Value(2), O::Input(0)),
                N::Divide(O::Value(3), O::Input(1)),
                N::Remainder(O::Value(3), O::Input(1)),
            ],
            &[1, 3, 4, 5],
        );
        assert!(!generic.native);
        let mut input = RetainedValues::empty();
        input.push_int(i64::MAX.into());
        input.push_int(0.into());
        let mut environment = BlockEnvironment::from_retained(input);
        let mut scratch = ArithmeticScratch::default();
        scratch.execute(&generic, &mut environment);
        assert_eq!(environment.int_ref(IntLocalId(2)).small(), None);
        assert_eq!(environment.int_ref(IntLocalId(3)).small(), Some(i64::MAX));
        assert_eq!(environment.int_ref(IntLocalId(4)).small(), Some(0));
        assert_eq!(environment.int_ref(IntLocalId(5)).small(), Some(0));
        let capacity = scratch.values.capacity();
        let empty = region(
            vec![N::Add(O::Input(0), O::Immediate(1)), N::Negate(O::Value(0))],
            &[],
        );
        scratch.execute(&empty, &mut environment);
        assert_eq!(scratch.values.capacity(), capacity);
    }
}
