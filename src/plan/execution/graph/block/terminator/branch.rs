use super::Edge;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::BoolLocalId;

pub(crate) struct BoolBranch {
    subject: BoolLocalId,
    true_: Edge,
    false_: Edge,
}

impl BoolBranch {
    pub(in crate::plan::execution) fn new(subject: BoolLocalId, true_: Edge, false_: Edge) -> Self {
        Self {
            subject,
            true_,
            false_,
        }
    }

    pub(crate) fn subject(&self) -> BoolLocalId {
        self.subject
    }

    pub(crate) fn true_(&self) -> &Edge {
        &self.true_
    }

    pub(crate) fn false_(&self) -> &Edge {
        &self.false_
    }
}

impl Explain for BoolBranch {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("branch ");
        context.write(&self.subject());
        context.push_str(" true=");
        context.write(self.true_());
        context.push_str(" false=");
        context.write(self.false_());
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::BoolBranch;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_bool_branch() {
        let source = r#"
fn identity(value: Bool) { value }

pub fn main() {
  case identity(True) {
    True -> 1
    False -> 0
  }
}
"#;
        let expected = "branch %bool#1 true=b1() false=b2()";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one Bool branch")]
    fn bool_branch_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            bool_branch(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one Bool branch")]
    fn bool_branch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: Bool) { value }

pub fn main() {
  case identity(True) {
    True -> 1
    False -> 0
  }
}
"#;
        explain::with_execution_plan(source, |plan| {
            let branch = bool_branch(&terminators(plan));
            bool_branch_from_nodes(&[branch, branch]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::graph::Terminator> {
        plan.int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .blocks()
            .iter()
            .map(|block| block.terminator())
            .collect()
    }

    fn bool_branch<'a>(
        terminators: &[&'a crate::plan::execution::graph::Terminator],
    ) -> &'a BoolBranch {
        let branches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::BoolBranch(branch) => Some(branch),
                _ => None,
            })
            .collect::<Vec<_>>();
        bool_branch_from_nodes(&branches)
    }

    fn bool_branch_from_nodes<'a>(branches: &[&'a BoolBranch]) -> &'a BoolBranch {
        let [branch] = branches else {
            panic!("source should lower one Bool branch");
        };
        branch
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let branch = bool_branch(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(branch);
        });
    }
}
