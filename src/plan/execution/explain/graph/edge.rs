use super::super::super::{Edge, MatchEdge, MatchEdgeArgument};
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

pub(super) fn write_match_edge(output: &mut String, edge: &MatchEdge) {
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
    use crate::plan::execution::{Edge, IntFunctionId, MatchEdge, Terminator};

    #[test]
    fn writes_regular_and_match_edge_argument_packs() {
        let source = r#"
fn choose(flag: Bool, value: Int) {
  case flag {
    True -> value
    False -> value + 1
  }
}

fn assert_head(values: List(Int)) {
  let assert [head, ..] = values
  head
}

pub fn main() { choose(True, assert_head([1])) }
"#;
        let plan = execution_plan(source);
        let choose = plan.int_function(IntFunctionId(2));
        let (true_, false_) = bool_branch_edges(choose.graph().blocks()[0].terminator());
        let mut output = String::new();
        super::write_edge(&mut output, true_);
        output.push(' ');
        super::write_edge(&mut output, false_);

        let assert_head = plan.int_function(IntFunctionId(1));
        let success = match_success_edge(assert_head.graph().blocks()[0].terminator());
        output.push(' ');
        super::write_match_edge(&mut output, success);

        assert_eq!(output, "b1(%int#0) b2(%int#0) b1(binding#0)");
    }

    #[test]
    #[should_panic(expected = "case should lower to a Bool branch")]
    fn bool_branch_edge_shape_guard_is_visible() {
        let plan = execution_plan("pub fn main() { 1 }");
        bool_branch_edges(plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator());
    }

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_edge_shape_guard_is_visible() {
        let plan = execution_plan("pub fn main() { 1 }");
        match_success_edge(plan.int_function(IntFunctionId(0)).graph().blocks()[0].terminator());
    }

    fn bool_branch_edges(terminator: &Terminator) -> (&Edge, &Edge) {
        let Terminator::BoolBranch { true_, false_, .. } = terminator else {
            panic!("case should lower to a Bool branch");
        };
        (true_, false_)
    }

    fn match_success_edge(terminator: &Terminator) -> &MatchEdge {
        let Terminator::Match { success, .. } = terminator else {
            panic!("let assert should lower to a match terminator");
        };
        success
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
