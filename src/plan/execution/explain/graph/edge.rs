mod match_;

pub(super) use match_::write_match_edge;

use super::super::super::Edge;
use super::super::value::ExplainLocal;

pub(super) fn write_edge(output: &mut String, edge: &Edge) {
    output.push('b');
    output.push_str(&edge.target().index().to_string());
    output.push('(');
    for (index, argument) in edge.args().iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        argument.write_local(output);
    }
    output.push(')');
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{Edge, IntFunctionId, Terminator};

    #[test]
    fn writes_regular_edge_argument_packs() {
        let source = r#"
fn identity(value: Bool) { value }

pub fn main() {
  let value = 1
  case identity(True) {
    True -> value
    False -> value + 1
  }
}
"#;
        let expected = "b1(%int#0) b2(%int#0)";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            let terminator = plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator();
            let (true_, false_) = bool_branch_edges(terminator);
            super::write_edge(output, true_);
            output.push(' ');
            super::write_edge(output, false_);
        });
    }

    fn bool_branch_edges(terminator: &Terminator) -> (&Edge, &Edge) {
        let Terminator::BoolBranch { true_, false_, .. } = terminator else {
            panic!("case should lower to a Bool branch");
        };
        (true_, false_)
    }

    #[test]
    #[should_panic(expected = "case should lower to a Bool branch")]
    fn bool_branch_edge_shape_guard_is_visible() {
        super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            bool_branch_edges(plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator());
        });
    }
}
