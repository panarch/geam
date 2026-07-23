use super::super::super::super::{MatchEdge, MatchEdgeArgument};
use super::super::super::value::ExplainLocal;

pub(in super::super) fn write_match_edge(output: &mut String, edge: &MatchEdge) {
    output.push('b');
    output.push_str(&edge.target().index().to_string());
    output.push('(');
    for (index, argument) in edge.args().iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        match argument {
            MatchEdgeArgument::Binding(binding) => {
                output.push_str("binding#");
                output.push_str(&binding.to_string());
            }
            MatchEdgeArgument::Value(value) => value.write_local(output),
        }
    }
    output.push(')');
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, MatchEdge, Terminator};

    #[test]
    fn writes_match_edge_argument_packs() {
        let source = r#"
pub fn main() {
  let values = [1]
  let assert [head, ..] = values
  head
}
"#;
        let expected = "b1(binding#0)";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let terminator = plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator();
            super::write_match_edge(output, match_success_edge(terminator));
        });
    }

    fn match_success_edge(terminator: &Terminator) -> &MatchEdge {
        let Terminator::Match { success, .. } = terminator else {
            panic!("let assert should lower to a match terminator");
        };
        success
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_edge_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            match_success_edge(
                plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator(),
            );
        });
    }
}
