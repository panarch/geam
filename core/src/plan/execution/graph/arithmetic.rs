use super::{IntLocalId, ParamSlot};
use crate::plan::execution::Table;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::prepared::rust::{Emit, Rust};

pub(crate) const MAX_ARITHMETIC_NODES: usize = 32;

/// A sealed straight-line integer program. Values are numbered in evaluation
/// order; only explicit outputs occupy the enclosing block's integer column.
#[derive(Clone)]
pub struct ArithmeticRegion {
    pub inputs: Table<IntLocalId>,
    pub nodes: Table<ArithmeticNode>,
    pub outputs: Table<ArithmeticOutput>,
    pub native: bool,
}

#[derive(Clone)]
pub struct ArithmeticOutput {
    pub value: usize,
    pub slot: ParamSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOperand {
    Input(usize),
    Value(usize),
    Immediate(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticNode {
    Add(ArithmeticOperand, ArithmeticOperand),
    Subtract(ArithmeticOperand, ArithmeticOperand),
    Multiply(ArithmeticOperand, ArithmeticOperand),
    Divide(ArithmeticOperand, ArithmeticOperand),
    Remainder(ArithmeticOperand, ArithmeticOperand),
    Negate(ArithmeticOperand),
}

impl ArithmeticNode {
    pub(crate) fn operands(&self) -> impl Iterator<Item = ArithmeticOperand> {
        let (left, right) = match *self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right)
            | Self::Remainder(left, right) => (left, Some(right)),
            Self::Negate(value) => (value, None),
        };
        std::iter::once(left).chain(right)
    }
}

#[derive(Clone, Copy)]
struct Interval {
    low: i128,
    high: i128,
}

/// Preparation and admission use the same bounded, conservative proof. No
/// runtime input or arbitrary-precision range analysis is part of this proof.
pub(crate) fn native_proof(inputs: usize, nodes: &[ArithmeticNode]) -> bool {
    let mut values = Vec::with_capacity(nodes.len());
    for node in nodes {
        let operand = |operand| match operand {
            ArithmeticOperand::Input(index) if index < inputs => Some(Interval {
                low: i128::from(i64::MIN),
                high: i128::from(i64::MAX),
            }),
            ArithmeticOperand::Input(_) => None,
            ArithmeticOperand::Value(index) => values.get(index).copied(),
            ArithmeticOperand::Immediate(value) => Some(Interval {
                low: i128::from(value),
                high: i128::from(value),
            }),
        };
        let result = match *node {
            ArithmeticNode::Add(left, right) => {
                operand(left).zip(operand(right)).and_then(|(left, right)| {
                    Some(Interval {
                        low: left.low.checked_add(right.low)?,
                        high: left.high.checked_add(right.high)?,
                    })
                })
            }
            ArithmeticNode::Subtract(left, right) => {
                operand(left).zip(operand(right)).and_then(|(left, right)| {
                    Some(Interval {
                        low: left.low.checked_sub(right.high)?,
                        high: left.high.checked_sub(right.low)?,
                    })
                })
            }
            ArithmeticNode::Multiply(left, right) => {
                operand(left).zip(operand(right)).and_then(|(left, right)| {
                    let corners = [
                        left.low.checked_mul(right.low)?,
                        left.low.checked_mul(right.high)?,
                        left.high.checked_mul(right.low)?,
                        left.high.checked_mul(right.high)?,
                    ];
                    Some(bounds(corners))
                })
            }
            ArithmeticNode::Divide(left, right) => operand(left)
                .zip(operand(right))
                .and_then(|(left, right)| divide(left, right)),
            ArithmeticNode::Remainder(left, right) => operand(left)
                .zip(operand(right))
                .and_then(|(left, right)| remainder(left, right)),
            ArithmeticNode::Negate(value) => operand(value).and_then(|value| {
                Some(Interval {
                    low: value.high.checked_neg()?,
                    high: value.low.checked_neg()?,
                })
            }),
        };
        let Some(result) = result else { return false };
        values.push(result);
    }
    true
}

fn bounds<const N: usize>(values: [i128; N]) -> Interval {
    values.into_iter().fold(
        Interval {
            low: i128::MAX,
            high: i128::MIN,
        },
        |range, value| Interval {
            low: range.low.min(value),
            high: range.high.max(value),
        },
    )
}

fn divide(left: Interval, right: Interval) -> Option<Interval> {
    let mut result = Interval { low: 0, high: 0 };
    // Endpoints plus the closest possible nonzero divisors enclose truncating
    // division, including a divisor interval crossing zero (which yields zero).
    for divisor in [right.low, right.high, -1, 1] {
        if divisor == 0 || divisor < right.low || divisor > right.high {
            continue;
        }
        for dividend in [left.low, left.high] {
            let value = dividend.checked_div(divisor)?;
            result.low = result.low.min(value);
            result.high = result.high.max(value);
        }
    }
    Some(result)
}

fn remainder(left: Interval, right: Interval) -> Option<Interval> {
    if left.low == i128::MIN && right.low <= -1 && right.high >= -1 {
        return None;
    }
    let divisor = right.low.unsigned_abs().max(right.high.unsigned_abs());
    if divisor == 0 {
        return Some(Interval { low: 0, high: 0 });
    }
    let magnitude = left
        .low
        .unsigned_abs()
        .max(left.high.unsigned_abs())
        .min(divisor - 1);
    // |divisor| is at most 2^127, so |remainder| <= |divisor| - 1
    // always fits i128. The exceptional MIN/-1 pair was rejected above.
    let magnitude = magnitude as i128;
    Some(Interval {
        low: if left.low < 0 { -magnitude } else { 0 },
        high: if left.high > 0 { magnitude } else { 0 },
    })
}

impl Explain for ArithmeticRegion {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("    arithmetic.region inputs=[");
        for (index, input) in self.inputs.iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            context.write(input);
        }
        context.push_str("] ");
        context.push_str(if self.native {
            "native=true\n"
        } else {
            "native=false\n"
        });
        for (index, node) in self.nodes.iter().enumerate() {
            context.push_str(&format!("      value#{index} = {node:?}\n"));
        }
        for output in &self.outputs {
            context.push_str("      ");
            context.write(&output.slot);
            context.push_str(&format!(" = value#{}\n", output.value));
        }
    }
}

impl Emit for ArithmeticRegion {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "graph::ArithmeticRegion",
            &[
                ("inputs", &self.inputs),
                ("nodes", &self.nodes),
                ("outputs", &self.outputs),
                ("native", &self.native),
            ],
        );
    }
}
impl Emit for ArithmeticOutput {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "graph::ArithmeticOutput",
            &[("value", &self.value), ("slot", &self.slot)],
        );
    }
}
impl Emit for ArithmeticOperand {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Input(value) => output.call("graph::ArithmeticOperand::Input", &[value]),
            Self::Value(value) => output.call("graph::ArithmeticOperand::Value", &[value]),
            Self::Immediate(value) => output.call("graph::ArithmeticOperand::Immediate", &[value]),
        }
    }
}
impl Emit for ArithmeticNode {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Add(left, right) => output.call("graph::ArithmeticNode::Add", &[left, right]),
            Self::Subtract(left, right) => {
                output.call("graph::ArithmeticNode::Subtract", &[left, right])
            }
            Self::Multiply(left, right) => {
                output.call("graph::ArithmeticNode::Multiply", &[left, right])
            }
            Self::Divide(left, right) => {
                output.call("graph::ArithmeticNode::Divide", &[left, right])
            }
            Self::Remainder(left, right) => {
                output.call("graph::ArithmeticNode::Remainder", &[left, right])
            }
            Self::Negate(value) => output.call("graph::ArithmeticNode::Negate", &[value]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArithmeticNode as N, ArithmeticOperand as O, native_proof};

    #[test]
    fn region_emission_and_explanation_fix_every_operator_operand_and_output() {
        use super::{ArithmeticOutput, ArithmeticRegion};
        use crate::plan::execution::explain::{Explain, ExplainContext};
        use crate::plan::execution::graph::{IntLocalId, ParamLocal, ParamSlot};
        use crate::plan::execution::prepared::rust::Rust;
        use crate::plan::execution::type_::ValueShapeId;
        let region = ArithmeticRegion {
            inputs: vec![IntLocalId(0), IntLocalId(1)].into(),
            nodes: vec![
                N::Add(O::Input(0), O::Input(1)),
                N::Subtract(O::Value(0), O::Immediate(3)),
                N::Multiply(O::Value(1), O::Value(1)),
                N::Divide(O::Value(2), O::Input(1)),
                N::Remainder(O::Value(3), O::Immediate(7)),
                N::Negate(O::Value(4)),
            ]
            .into(),
            outputs: vec![
                ArithmeticOutput {
                    value: 0,
                    slot: ParamSlot::new(ParamLocal::Int(IntLocalId(2)), ValueShapeId(0)),
                },
                ArithmeticOutput {
                    value: 5,
                    slot: ParamSlot::new(ParamLocal::Int(IntLocalId(3)), ValueShapeId(0)),
                },
            ]
            .into(),
            native: false,
        };
        assert!(!native_proof(2, &region.nodes));
        assert_eq!(
            Rust::expression(&region),
            r#"data::graph::ArithmeticRegion {
    inputs: data::Storage::Static(&[
        data::graph::IntLocalId(0),
        data::graph::IntLocalId(1),
    ]),
    nodes: data::Storage::Static(&[
        data::graph::ArithmeticNode::Add(data::graph::ArithmeticOperand::Input(0), data::graph::ArithmeticOperand::Input(1)),
        data::graph::ArithmeticNode::Subtract(data::graph::ArithmeticOperand::Value(0), data::graph::ArithmeticOperand::Immediate(3)),
        data::graph::ArithmeticNode::Multiply(data::graph::ArithmeticOperand::Value(1), data::graph::ArithmeticOperand::Value(1)),
        data::graph::ArithmeticNode::Divide(data::graph::ArithmeticOperand::Value(2), data::graph::ArithmeticOperand::Input(1)),
        data::graph::ArithmeticNode::Remainder(data::graph::ArithmeticOperand::Value(3), data::graph::ArithmeticOperand::Immediate(7)),
        data::graph::ArithmeticNode::Negate(data::graph::ArithmeticOperand::Value(4)),
    ]),
    outputs: data::Storage::Static(&[
        data::graph::ArithmeticOutput {
            value: 0,
            slot: data::graph::ParamSlot {
                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
                shape: data::type_::ValueShapeId(0),
            },
        },
        data::graph::ArithmeticOutput {
            value: 5,
            slot: data::graph::ParamSlot {
                local: data::graph::ParamLocal::Int(data::graph::IntLocalId(3)),
                shape: data::type_::ValueShapeId(0),
            },
        },
    ]),
    native: false,
}"#
        );
        let typed =
            crate::compile_typed_module("main", "main.gleam", "pub fn main() { 1 }").unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let mut explanation = String::new();
        region.write_explanation(&mut ExplainContext::new(&plan, &mut explanation));
        assert_eq!(
            explanation,
            "    arithmetic.region inputs=[%int#0, %int#1] native=false\n      value#0 = Add(Input(0), Input(1))\n      value#1 = Subtract(Value(0), Immediate(3))\n      value#2 = Multiply(Value(1), Value(1))\n      value#3 = Divide(Value(2), Input(1))\n      value#4 = Remainder(Value(3), Immediate(7))\n      value#5 = Negate(Value(4))\n      %int#2:shape#0(Int) = value#0\n      %int#3:shape#0(Int) = value#5\n"
        );
    }

    #[test]
    fn proof_rejects_overflow_at_each_interval_endpoint() {
        let square = N::Multiply(O::Immediate(i64::MIN), O::Immediate(i64::MIN));
        let variable = N::Multiply(O::Input(0), O::Input(1));
        // The low endpoint of the sum fits, while the high endpoint is 2^127.
        assert!(!native_proof(
            2,
            &[variable, N::Add(O::Value(0), O::Value(0))]
        ));
        assert!(!native_proof(
            2,
            &[
                variable,
                square,
                N::Negate(O::Value(1)),
                N::Subtract(O::Value(0), O::Value(2)),
            ]
        ));
        // Division's conservative interval includes zero. These cases isolate
        // the remaining multiplication corners without overflowing an earlier one.
        assert!(!native_proof(
            0,
            &[
                square,
                N::Divide(O::Immediate(2), O::Immediate(1)),
                N::Multiply(O::Value(0), O::Value(1)),
            ]
        ));
        assert!(!native_proof(
            0,
            &[
                square,
                N::Negate(O::Value(0)),
                N::Subtract(O::Value(1), O::Immediate(1)),
                N::Divide(O::Immediate(2), O::Immediate(1)),
                N::Multiply(O::Value(3), O::Value(2)),
            ]
        ));
        assert!(!native_proof(
            0,
            &[
                square,
                N::Divide(O::Value(0), O::Immediate(1)),
                N::Multiply(O::Value(1), O::Value(1)),
            ]
        ));
        assert!(!native_proof(
            0,
            &[
                square,
                N::Negate(O::Value(0)),
                N::Multiply(O::Value(1), O::Immediate(2)),
                N::Divide(O::Value(2), O::Immediate(1)),
                N::Negate(O::Value(3)),
            ]
        ));
    }

    #[test]
    fn native_proof_encloses_all_operations_and_divisor_signs() {
        let nodes = [
            N::Add(O::Input(0), O::Immediate(i64::MAX)),
            N::Subtract(O::Value(0), O::Immediate(i64::MIN)),
            N::Multiply(O::Input(0), O::Immediate(2)),
            N::Divide(O::Value(2), O::Input(1)),
            N::Remainder(O::Value(3), O::Input(1)),
            N::Negate(O::Value(4)),
        ];
        assert!(native_proof(2, &nodes));
        assert!(native_proof(
            0,
            &[
                N::Divide(O::Immediate(1), O::Immediate(0)),
                N::Remainder(O::Value(0), O::Immediate(0))
            ]
        ));
        assert!(native_proof(
            0,
            &[
                N::Divide(O::Immediate(9), O::Immediate(2)),
                N::Remainder(O::Value(0), O::Immediate(3))
            ]
        ));
    }

    #[test]
    fn proof_rejects_unknown_indices_overflow_and_signed_division_exception() {
        assert!(!native_proof(0, &[N::Negate(O::Input(0))]));
        assert!(!native_proof(0, &[N::Negate(O::Value(0))]));
        let square = N::Multiply(O::Immediate(i64::MIN), O::Immediate(i64::MIN));
        assert!(!native_proof(
            0,
            &[square, N::Multiply(O::Value(0), O::Value(0))]
        ));
        let minimum = [
            square,
            N::Negate(O::Value(0)),
            N::Multiply(O::Value(1), O::Immediate(2)),
        ];
        for node in [
            N::Divide(O::Value(2), O::Immediate(-1)),
            N::Remainder(O::Value(2), O::Immediate(-1)),
            N::Negate(O::Value(2)),
            N::Subtract(O::Value(2), O::Immediate(1)),
            N::Add(O::Value(2), O::Immediate(-1)),
        ] {
            let mut nodes = minimum.to_vec();
            nodes.push(node);
            assert!(!native_proof(0, &nodes));
        }
        let mut nodes = minimum.to_vec();
        nodes.push(N::Remainder(O::Value(2), O::Value(2)));
        assert!(native_proof(0, &nodes));
    }
}
