use super::super::draft::instruction::DraftBoolInstruction;
use super::super::draft::{
    DraftBlock, DraftBoolTest, DraftGraph, DraftInstruction, DraftTerminator, DraftValueKey,
};
use super::value::BlockValues;
use crate::plan::execution::graph::BoolTest;
use std::collections::HashMap;

pub(super) fn fuse(graph: &mut DraftGraph) {
    let mut counts = HashMap::<DraftValueKey, usize>::new();
    let mut uses = Vec::<DraftValueKey>::new();
    for block in graph.blocks.values() {
        for instruction in &block.instructions {
            instruction.uses(&mut uses);
        }
        block.terminator.uses(&mut uses);
        for value in uses.drain(..) {
            *counts.entry(value).or_default() += 1;
        }
    }
    graph.blocks = std::mem::take(&mut graph.blocks)
        .into_iter()
        .map(|(id, block)| (id, fuse_block(block, &counts)))
        .collect();
}

pub(super) fn freeze(test: &DraftBoolTest, values: &BlockValues) -> BoolTest {
    use BoolTest as E;
    match test {
        DraftBoolTest::Not(value) => E::Not(values.bool(value)),
        DraftBoolTest::EqualInt { left, right } => E::EqualInt {
            left: values.integer_operand(left),
            right: values.integer_operand(right),
        },
        DraftBoolTest::NotEqualInt { left, right } => E::NotEqualInt {
            left: values.integer_operand(left),
            right: values.integer_operand(right),
        },
        DraftBoolTest::LtInt { left, right } => E::LtInt {
            left: values.integer_operand(left),
            right: values.integer_operand(right),
        },
        DraftBoolTest::LtEqInt { left, right } => E::LtEqInt {
            left: values.integer_operand(left),
            right: values.integer_operand(right),
        },
        DraftBoolTest::GtInt { left, right } => E::GtInt {
            left: values.integer_operand(left),
            right: values.integer_operand(right),
        },
        DraftBoolTest::GtEqInt { left, right } => E::GtEqInt {
            left: values.integer_operand(left),
            right: values.integer_operand(right),
        },
        DraftBoolTest::LtFloat { left, right } => E::LtFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolTest::LtEqFloat { left, right } => E::LtEqFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolTest::GtFloat { left, right } => E::GtFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolTest::GtEqFloat { left, right } => E::GtEqFloat {
            left: values.float(left),
            right: values.float(right),
        },
        DraftBoolTest::Equal { left, right } => E::Equal {
            left: values.any(left),
            right: values.any(right),
        },
        DraftBoolTest::NotEqual { left, right } => E::NotEqual {
            left: values.any(left),
            right: values.any(right),
        },
        DraftBoolTest::StringStartsWith { value, prefix } => E::StringStartsWith {
            value: values.string(value),
            prefix: prefix.clone().into(),
        },
        DraftBoolTest::ListLengthEquals { value, length } => E::ListLengthEquals {
            value: values.list(value),
            length: *length,
        },
        DraftBoolTest::ListLengthAtLeast { value, length } => E::ListLengthAtLeast {
            value: values.list(value),
            length: *length,
        },
    }
}

fn fuse_block(mut block: DraftBlock, counts: &HashMap<DraftValueKey, usize>) -> DraftBlock {
    match (block.instructions.pop(), block.terminator) {
        (
            Some(DraftInstruction::Bool {
                output,
                kind: DraftBoolInstruction::Test(test),
            }),
            DraftTerminator::BoolBranch {
                subject,
                true_,
                false_,
            },
        ) if output.key == subject.key && counts.get(&output.key) == Some(&1) => {
            block.terminator = DraftTerminator::TestBranch {
                test,
                true_,
                false_,
            };
        }
        (last, terminator) => {
            if let Some(last) = last {
                block.instructions.push(last);
            }
            block.terminator = terminator;
        }
    }
    block
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, BoolBranch, BoolLocalId, Echo, IntLocalId, ParamLocal,
        ProfiledInstruction, Terminator, TestBranch,
    };
    use std::convert::Infallible;

    #[test]
    fn adjacent_single_use_test_has_no_output_slot_and_keeps_its_edges() {
        explain::with_execution_plan(
            "pub fn main() { case 42 < 100 { True -> 1 False -> 0 } }",
            |plan| {
                let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
                let entry = graph.block(graph.entry());
                assert!(entry.params().is_empty());
                assert!(entry.instructions().is_empty());
                let branch = direct_branch(entry.terminator());
                let mut actual = String::new();
                let mut context = explain::ExplainContext::new(plan, &mut actual);
                context.write(&branch.test);
                assert_eq!(actual, "bool.lt_int 42 100");
                assert_eq!(
                    (branch.true_.target(), branch.false_.target()),
                    (BlockId(1), BlockId(2))
                );
                assert_eq!(
                    (branch.true_.args(), branch.false_.args()),
                    (&[][..], &[][..])
                );
            },
        );
    }

    #[test]
    fn a_returned_branch_subject_retains_its_bool_output_and_edge_argument() {
        for source in [
            "fn choose(value: Int) { let valid = value < 10 case valid { True -> valid False -> False } } pub fn main() { choose(4) }",
            "fn choose(value: Int) { let valid = value < 10 case valid { True -> { let callback = fn() { valid } callback() } False -> False } } pub fn main() { choose(4) }",
        ] {
            explain::with_execution_plan(source, |plan| {
                let graph = plan.bool_function(BoolFunctionId(1)).body().block_graph();
                let entry = graph.block(graph.entry());
                assert_eq!(
                    entry
                        .params()
                        .iter()
                        .map(|slot| &slot.local)
                        .collect::<Vec<_>>(),
                    [&ParamLocal::Int(IntLocalId(0))]
                );
                let instruction = sole_instruction(entry.instructions()).value().unwrap();
                assert_eq!(instruction.output.local, ParamLocal::Bool(BoolLocalId(0)));
                let mut actual = String::new();
                let mut context = explain::ExplainContext::new(plan, &mut actual);
                context.write(&instruction.kind);
                assert_eq!(actual, "bool.lt_int %int#0 10");
                let branch = value_branch(entry.terminator());
                assert_eq!(branch.subject, BoolLocalId(0));
                assert_eq!(branch.true_.args(), &[ParamLocal::Bool(BoolLocalId(0))]);
                assert_eq!(branch.false_.args(), &[]);
            });
        }
    }

    #[test]
    fn an_echo_between_test_and_branch_keeps_the_value_and_evaluation_order() {
        let source = "fn choose(value: Int) { let valid = value < 10 let _ = echo value case valid { True -> 1 False -> 0 } } pub fn main() { choose(4) }";
        explain::with_execution_plan(source, |plan| {
            let graph = plan.int_function(IntFunctionId(1)).body().block_graph();
            let entry = graph.block(graph.entry());
            let instruction = sole_instruction(entry.instructions()).value().unwrap();
            let mut actual = String::new();
            let mut context = explain::ExplainContext::new(plan, &mut actual);
            context.write(&instruction.kind);
            assert_eq!(actual, "bool.lt_int %int#0 10");
            let echo = echo(entry.terminator());
            assert_eq!(echo.next.args(), &[ParamLocal::Bool(BoolLocalId(0))]);
            let next = graph.block(echo.next.target());
            assert!(next.instructions().is_empty());
            assert!(
                matches!(next.terminator(), Terminator::BoolBranch(branch) if branch.subject == BoolLocalId(0))
            );
        });
    }

    fn direct_branch(terminator: &Terminator) -> &TestBranch {
        match terminator {
            Terminator::TestBranch(branch) => branch,
            _ => panic!("fixture should contain a direct test branch"),
        }
    }

    fn value_branch(terminator: &Terminator) -> &BoolBranch {
        match terminator {
            Terminator::BoolBranch(branch) => branch,
            _ => panic!("fixture should retain a Bool branch"),
        }
    }

    fn echo(terminator: &Terminator) -> &Echo {
        match terminator {
            Terminator::Echo(echo) => echo,
            _ => panic!("fixture should contain an Echo"),
        }
    }

    fn sole_instruction(
        instructions: &[ProfiledInstruction<Infallible>],
    ) -> &ProfiledInstruction<Infallible> {
        match instructions {
            [instruction] => instruction,
            _ => panic!("fixture should contain exactly one instruction"),
        }
    }

    #[test]
    #[should_panic(expected = "fixture should contain a direct test branch")]
    fn direct_branch_fixture_rejects_an_exit() {
        direct_branch(&Terminator::Exit(BlockGraphExitId(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should retain a Bool branch")]
    fn value_branch_fixture_rejects_an_exit() {
        value_branch(&Terminator::Exit(BlockGraphExitId(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain an Echo")]
    fn echo_fixture_rejects_an_exit() {
        echo(&Terminator::Exit(BlockGraphExitId(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain exactly one instruction")]
    fn sole_instruction_fixture_rejects_empty_instructions() {
        sole_instruction(&[]);
    }
}
