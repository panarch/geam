use super::super::super::value::ExplainLocal;
use super::super::edge::write_edge;
use crate::plan::execution::{BoolLocalId, Edge};

pub(super) fn write_bool_branch(
    output: &mut String,
    subject: &BoolLocalId,
    true_: &Edge,
    false_: &Edge,
) {
    output.push_str("branch ");
    subject.write_local(output);
    output.push_str(" true=");
    write_edge(output, true_);
    output.push_str(" false=");
    write_edge(output, false_);
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_bool_branch() {
        assert_explanation(
            r#"
fn identity(value: Bool) { value }

pub fn main() {
  case identity(True) {
    True -> 1
    False -> 0
  }
}
"#,
            "branch %bool#1 true=b1() false=b2()",
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            let (subject, true_, false_) = bool_branch(&terminators);
            super::write_bool_branch(output, subject, true_, false_);
        });
    }

    fn bool_branch<'a>(
        terminators: &[&'a Terminator],
    ) -> (
        &'a crate::plan::execution::BoolLocalId,
        &'a crate::plan::execution::Edge,
        &'a crate::plan::execution::Edge,
    ) {
        let mut branches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::BoolBranch {
                    subject,
                    true_,
                    false_,
                } => Some((subject, true_, false_)),
                _ => None,
            });
        let Some(branch) = branches.next() else {
            panic!("source should lower one Bool branch");
        };
        if branches.next().is_some() {
            panic!("source should lower one Bool branch");
        }
        branch
    }

    #[test]
    #[should_panic(expected = "source should lower one Bool branch")]
    fn bool_branch_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            bool_branch(&terminators);
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
        super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::BoolBranch { .. }))
                .expect("source should lower a Bool branch");
            bool_branch(&[terminator, terminator]);
        });
    }
}
