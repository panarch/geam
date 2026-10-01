use super::super::draft::DraftBoolTest;
use super::super::draft::instruction::{
    DraftBoolInstruction, DraftIntInstruction, DraftIntegerOperand,
};
use super::super::draft::{DraftGraphBuilder, DraftGraphValue, DraftInstruction, DraftValueKey};
use std::collections::{HashMap, HashSet};

pub(super) fn use_immediates<Return: DraftGraphValue, TailCall>(
    graph: &mut DraftGraphBuilder<Return, TailCall>,
) {
    let mut replaced = HashSet::new();
    for block in graph.graph.blocks.values_mut() {
        let mut literals = HashMap::new();
        for instruction in &mut block.instructions {
            match instruction {
                DraftInstruction::Int {
                    output,
                    kind: DraftIntInstruction::Value(value),
                } => {
                    if let Ok(value) = i64::try_from(&*value) {
                        literals.insert(output.key, value);
                    }
                }
                DraftInstruction::Int { kind, .. } => {
                    int(kind, &literals, &mut replaced);
                }
                DraftInstruction::Bool { kind, .. } => {
                    bool(kind, &literals, &mut replaced);
                }
                DraftInstruction::IntegerRegion(_)
                | DraftInstruction::Float { .. }
                | DraftInstruction::String { .. }
                | DraftInstruction::BitArray { .. }
                | DraftInstruction::UtfCodepoint { .. }
                | DraftInstruction::Custom { .. }
                | DraftInstruction::External { .. }
                | DraftInstruction::Nil { .. }
                | DraftInstruction::Tuple { .. }
                | DraftInstruction::List { .. }
                | DraftInstruction::Function { .. } => {}
            }
        }
    }
    if replaced.is_empty() {
        return;
    }

    // Keep definitions used anywhere after rewriting, including another block.
    // The scratch vector retains at most one block's references.
    let mut uses = Vec::<DraftValueKey>::new();
    for block in graph.graph.blocks.values() {
        for instruction in &block.instructions {
            instruction.uses(&mut uses);
        }
        block.terminator.uses(&mut uses);
        for value in uses.drain(..) {
            replaced.remove(&value);
        }
    }
    for value in &graph.returns {
        replaced.remove(&value.key());
    }
    // Tail-call table entries are function identities and source sites. Their
    // value arguments are visited in DraftTerminator::uses above.
    for block in graph.graph.blocks.values_mut() {
        block.instructions.retain(|instruction| {
            !matches!(instruction,
                DraftInstruction::Int { output, kind: DraftIntInstruction::Value(_) }
                    if replaced.contains(&output.key)
            )
        });
    }
}

fn int(
    instruction: &mut DraftIntInstruction,
    literals: &HashMap<DraftValueKey, i64>,
    replaced: &mut HashSet<DraftValueKey>,
) {
    match instruction {
        DraftIntInstruction::Add { left, right }
        | DraftIntInstruction::Sub { left, right }
        | DraftIntInstruction::Mult { left, right }
        | DraftIntInstruction::Div { left, right }
        | DraftIntInstruction::Remainder { left, right } => {
            operand(left, literals, replaced);
            operand(right, literals, replaced);
        }
        DraftIntInstruction::Value(_)
        | DraftIntInstruction::Constant(_)
        | DraftIntInstruction::Call { .. }
        | DraftIntInstruction::FunctionCall { .. }
        | DraftIntInstruction::TupleIndex { .. }
        | DraftIntInstruction::CustomField { .. }
        | DraftIntInstruction::ListIndex { .. }
        | DraftIntInstruction::Negate(_) => {}
    }
}

fn bool(
    instruction: &mut DraftBoolInstruction,
    literals: &HashMap<DraftValueKey, i64>,
    replaced: &mut HashSet<DraftValueKey>,
) {
    if let DraftBoolInstruction::Test(test) = instruction {
        match test {
            DraftBoolTest::EqualInt { left, right }
            | DraftBoolTest::NotEqualInt { left, right }
            | DraftBoolTest::LtInt { left, right }
            | DraftBoolTest::LtEqInt { left, right }
            | DraftBoolTest::GtInt { left, right }
            | DraftBoolTest::GtEqInt { left, right } => {
                operand(left, literals, replaced);
                operand(right, literals, replaced);
            }
            DraftBoolTest::Not(_)
            | DraftBoolTest::LtFloat { .. }
            | DraftBoolTest::LtEqFloat { .. }
            | DraftBoolTest::GtFloat { .. }
            | DraftBoolTest::GtEqFloat { .. }
            | DraftBoolTest::Equal { .. }
            | DraftBoolTest::NotEqual { .. }
            | DraftBoolTest::StringStartsWith { .. }
            | DraftBoolTest::ListLengthEquals { .. }
            | DraftBoolTest::ListLengthAtLeast { .. } => {}
        }
    }
}

fn operand(
    operand: &mut DraftIntegerOperand,
    literals: &HashMap<DraftValueKey, i64>,
    replaced: &mut HashSet<DraftValueKey>,
) {
    match operand {
        DraftIntegerOperand::Local(local) => {
            if let Some(value) = literals.get(&local.key) {
                replaced.insert(local.key);
                *operand = DraftIntegerOperand::Immediate(*value);
            }
        }
        DraftIntegerOperand::Immediate(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DraftGraphBuilder, DraftInstruction, DraftIntInstruction, DraftIntegerOperand,
        use_immediates,
    };
    use crate::plan::execution::explain::{self, ExplainContext};
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::lowering::graph::draft::DraftInt;
    use crate::plan::execution::lowering::graph::freeze::freeze_constant;
    use crate::plan::execution::lowering::specialization::SpecializedValueShape;
    use crate::plan::execution::lowering::test_support::lowering_context;
    use crate::plan::execution::prepared::rust::Rust;
    use std::convert::Infallible;

    #[test]
    fn removes_only_rewritten_definitions_and_keeps_returned_and_unused_values() {
        let (mut graph, mut cursor) =
            DraftGraphBuilder::<DraftInt, Infallible>::new(vec![], vec![]);
        let kept = graph.int_instruction(&mut cursor, DraftIntInstruction::Value(10.into()));
        let removed = graph.int_instruction(&mut cursor, DraftIntInstruction::Value(20.into()));
        let unused = graph.int_instruction(&mut cursor, DraftIntInstruction::Value(99.into()));
        let sum = graph.int_instruction(
            &mut cursor,
            DraftIntInstruction::Add {
                left: DraftIntegerOperand::Local(kept.clone()),
                right: DraftIntegerOperand::Local(removed.clone()),
            },
        );
        graph.finish_return(cursor, kept.clone());
        use_immediates(&mut graph);
        let block = &graph.graph.blocks[&graph.graph.entry];
        assert_eq!(
            block
                .instructions
                .iter()
                .flat_map(|instruction| instruction.outputs().map(|output| output.key))
                .collect::<Vec<_>>(),
            [kept.key, unused.key, sum.key]
        );
        assert_eq!(graph.returns[0].key, kept.key);
        use_immediates(&mut graph);
        assert_eq!(graph.graph.blocks[&graph.graph.entry].instructions.len(), 3);
        let constant = freeze_constant(
            graph,
            &SpecializedValueShape::Int,
            &mut lowering_context(vec![]),
        );
        let block = constant.block_graph().block(constant.block_graph().entry());
        assert_eq!(
            Rust::expression(block.instructions()[2].value().unwrap().kind()),
            "data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {\n    left: data::graph::IntegerOperand::Immediate(10),\n    right: data::graph::IntegerOperand::Immediate(20),\n})"
        );
    }

    #[test]
    fn rewrites_same_block_uses_but_preserves_references_in_other_blocks() {
        let (mut graph, mut entry) = DraftGraphBuilder::<DraftInt, usize>::new(vec![], vec![]);
        let kept = graph.int_instruction(&mut entry, DraftIntInstruction::Value(10.into()));
        let removed = graph.int_instruction(&mut entry, DraftIntInstruction::Value(20.into()));
        let sum = graph.int_instruction(
            &mut entry,
            DraftIntInstruction::Add {
                left: DraftIntegerOperand::Local(kept.clone()),
                right: DraftIntegerOperand::Local(removed),
            },
        );
        let mut next = graph.empty_block(entry.scope().clone());
        let next_id = next.id();
        let result = graph.int_instruction(
            &mut next,
            DraftIntInstruction::Sub {
                left: DraftIntegerOperand::Local(sum.clone()),
                right: DraftIntegerOperand::Local(kept.clone()),
            },
        );
        graph.finish_return(next, result);
        graph.finish_jump(entry, next_id, vec![]);
        use_immediates(&mut graph);
        let entry = &graph.graph.blocks[&graph.graph.entry];
        assert_eq!(
            entry
                .instructions
                .iter()
                .flat_map(|instruction| instruction.outputs().map(|output| output.key))
                .collect::<Vec<_>>(),
            [kept.key, sum.key]
        );
        let next = &graph.graph.blocks[&next_id];
        assert!(matches!(&next.instructions[0], DraftInstruction::Int {
            kind: DraftIntInstruction::Sub {
                left: DraftIntegerOperand::Local(left), right: DraftIntegerOperand::Local(right),
            }, ..
        } if left.key == sum.key && right.key == kept.key));
    }

    #[test]
    fn source_aliases_large_values_constants_and_retained_arguments_have_exact_operands() {
        for (source, expected) in [
            ("pub fn main() { 1 + 2 }", "int.add 1 2"),
            (
                "pub fn main() { let x = 5 let alias = x let _ = alias + 1 x - 2 }",
                "    arithmetic.region inputs=[] native=true\n      value#0 = Add(Immediate(5), Immediate(1))\n      value#1 = Subtract(Immediate(5), Immediate(2))\n      %int#0:shape#0(Int) = value#1\n",
            ),
            (
                "pub fn main() { let x = 5 let _ = x + 1 x }",
                "int.value 5 | int.add 5 1",
            ),
            (
                "fn keep(x: Int) { x } pub fn main() { let x = 5 let _ = x + 1 let result = keep(x) result + 2 }",
                "int.value 5 | int.add 5 1 | int.call int#1 args=[%int#0] | int.add %int#2 2",
            ),
            (
                "fn keep(x: Int) { x } pub fn main() { let x = 5 let _ = x + 1 keep(x) }",
                "int.value 5 | int.add 5 1",
            ),
            (
                "pub fn main() { let x = 5 let _ = x + 1 let f = fn() { x } f() }",
                "int.value 5 | int.add 5 1 | function[Int] closure target=int#1 captures=[%int#0<-%int#0] | int.function_call %function.int#0 args=[]",
            ),
            (
                "pub fn main() { 9223372036854775808 + 1 }",
                "int.value 9223372036854775808 | int.add %int#0 1",
            ),
            (
                "pub fn main() { -9223372036854775809 - 1 }",
                "int.value -9223372036854775809 | int.sub %int#0 1",
            ),
            (
                "const offset = 5 pub fn main() { offset + 1 }",
                "constant.int#0 | int.add %int#0 1",
            ),
        ] {
            explain::assert_rendered(source, expected, |plan, output| {
                let body = plan.int_function(IntFunctionId(0)).body();
                for (index, instruction) in body
                    .block_graph()
                    .blocks()
                    .flat_map(|block| block.instructions())
                    .enumerate()
                {
                    if index > 0 {
                        output.push_str(" | ");
                    }
                    match instruction {
                        crate::plan::execution::graph::ProfiledInstruction::Value(value) => {
                            ExplainContext::new(plan, output).write(value.kind())
                        }
                        crate::plan::execution::graph::ProfiledInstruction::IntegerRegion(
                            region,
                        ) => ExplainContext::new(plan, output).write(region),
                    }
                }
            });
        }
    }

    #[test]
    fn constant_graphs_use_the_same_rewrite_before_slot_allocation() {
        let (mut graph, mut cursor) =
            DraftGraphBuilder::<DraftInt, Infallible>::new(vec![], vec![]);
        let left = graph.int_instruction(&mut cursor, DraftIntInstruction::Value(20.into()));
        let right = graph.int_instruction(&mut cursor, DraftIntInstruction::Value(22.into()));
        let result = graph.int_instruction(
            &mut cursor,
            DraftIntInstruction::Add {
                left: DraftIntegerOperand::Local(left),
                right: DraftIntegerOperand::Local(right),
            },
        );
        graph.finish_return(cursor, result);
        let constant = freeze_constant(
            graph,
            &SpecializedValueShape::Int,
            &mut lowering_context(vec![]),
        );
        let block = constant.block_graph().block(constant.block_graph().entry());
        assert_eq!(block.instructions().len(), 1);
        assert_eq!(
            Rust::expression(block.instructions()[0].value().unwrap().kind()),
            "data::graph::ProfiledInstructionKind::Int(data::graph::IntInstruction::Add {\n    left: data::graph::IntegerOperand::Immediate(20),\n    right: data::graph::IntegerOperand::Immediate(22),\n})"
        );
    }
}
