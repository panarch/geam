use super::Edge;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::BoolTest;
use crate::plan::execution::prepared::rust::{Emit, Rust};

#[derive(Clone)]
pub struct TestBranch {
    pub test: BoolTest,
    pub true_: Edge,
    pub false_: Edge,
}

impl TestBranch {
    pub(in crate::plan::execution) fn new(test: BoolTest, true_: Edge, false_: Edge) -> Self {
        Self {
            test,
            true_,
            false_,
        }
    }
}

impl Explain for TestBranch {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("branch_test ");
        context.write(&self.test);
        context.push_str(" true=");
        context.write(&self.true_);
        context.push_str(" false=");
        context.write(&self.false_);
    }
}

impl Emit for TestBranch {
    fn emit(&self, output: &mut Rust) {
        output.structure(
            "graph::TestBranch",
            &[
                ("test", &self.test),
                ("true_", &self.true_),
                ("false_", &self.false_),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{BoolTest, TestBranch};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, Edge, IntegerOperand, Terminator, Transfer,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::storage::Table;

    #[test]
    fn emits_the_test_and_two_distinct_edges() {
        let branch = TestBranch::new(
            BoolTest::LtInt {
                left: IntegerOperand::Immediate(-2),
                right: IntegerOperand::Immediate(3),
            },
            Edge::new(
                BlockId(1),
                Vec::new(),
                Transfer {
                    families: Table::Static(&[]),
                },
            ),
            Edge::new(
                BlockId(2),
                Vec::new(),
                Transfer {
                    families: Table::Static(&[]),
                },
            ),
        );
        assert_eq!(
            Rust::expression(&branch),
            r#"data::graph::TestBranch {
    test: data::graph::BoolTest::LtInt {
        left: data::graph::IntegerOperand::Immediate(-2),
        right: data::graph::IntegerOperand::Immediate(3),
    },
    true_: data::graph::Edge {
        target: data::graph::BlockId(1),
        args: data::Storage::Static(&[]),
        transfer: data::graph::Transfer {
            families: data::Storage::Static(&[]),
        },
    },
    false_: data::graph::Edge {
        target: data::graph::BlockId(2),
        args: data::Storage::Static(&[]),
        transfer: data::graph::Transfer {
            families: data::Storage::Static(&[]),
        },
    },
}"#
        );
    }

    #[test]
    fn explains_the_direct_test_and_edges_from_source() {
        explain::assert_rendered(
            "pub fn main() { case 42 < 100 { True -> 1 False -> 0 } }",
            "branch_test bool.lt_int 42 100 true=b1() false=b2()",
            |plan, output| {
                let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
                let branch = direct_branch(graph.block(graph.entry()).terminator());
                let mut context = explain::ExplainContext::new(plan, output);
                context.write(branch);
            },
        );
    }

    fn direct_branch(terminator: &Terminator) -> &TestBranch {
        match terminator {
            Terminator::TestBranch(branch) => branch,
            _ => panic!("source must produce a direct test"),
        }
    }

    #[test]
    #[should_panic(expected = "source must produce a direct test")]
    fn source_fixture_rejects_an_exit() {
        direct_branch(&Terminator::Exit(BlockGraphExitId(0)));
    }
}
