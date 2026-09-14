use super::super::BlockId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub struct Edge {
    pub target: BlockId,
    pub args: Table<ParamLocal>,
}

#[derive(Clone)]
pub struct MatchEdge {
    pub target: BlockId,
    pub args: Table<MatchEdgeArgument>,
}

#[derive(Clone)]
pub enum MatchEdgeArgument {
    Binding(usize),
    Value(ParamLocal),
}

impl Edge {
    pub(in crate::plan::execution) fn new(target: BlockId, args: Vec<ParamLocal>) -> Self {
        Self {
            target,
            args: args.into(),
        }
    }

    pub(crate) fn target(&self) -> BlockId {
        self.target
    }

    pub(crate) fn args(&self) -> &[ParamLocal] {
        &self.args
    }
}

impl MatchEdge {
    pub(in crate::plan::execution) fn new(target: BlockId, args: Vec<MatchEdgeArgument>) -> Self {
        Self {
            target,
            args: args.into(),
        }
    }

    pub(crate) fn target(&self) -> BlockId {
        self.target
    }

    pub(crate) fn args(&self) -> &[MatchEdgeArgument] {
        &self.args
    }
}

impl Explain for Edge {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push('b');
        context.push_str(&self.target().index().to_string());
        context.push('(');
        for (index, argument) in self.args().iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            argument.write_local_label(context.output());
        }
        context.push(')');
    }
}

impl Explain for MatchEdge {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push('b');
        context.push_str(&self.target().index().to_string());
        context.push('(');
        for (index, argument) in self.args().iter().enumerate() {
            if index > 0 {
                context.push_str(", ");
            }
            match argument {
                MatchEdgeArgument::Binding(binding) => {
                    context.push_str("binding#");
                    context.push_str(&binding.to_string());
                }
                MatchEdgeArgument::Value(value) => value.write_local_label(context.output()),
            }
        }
        context.push(')');
    }
}

impl Emit for Edge {
    fn emit(&self, output: &mut Rust) {
        let Self { target, args } = self;
        output.structure("graph::Edge", &[("target", target), ("args", args)]);
    }
}

impl Emit for MatchEdge {
    fn emit(&self, output: &mut Rust) {
        let Self { target, args } = self;
        output.structure("graph::MatchEdge", &[("target", target), ("args", args)]);
    }
}

impl Emit for MatchEdgeArgument {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Binding(field_0) => output.call("graph::MatchEdgeArgument::Binding", &[field_0]),
            Self::Value(field_0) => output.call("graph::MatchEdgeArgument::Value", &[field_0]),
        }
    }
}

#[cfg(test)]
mod edge_explain_tests {
    use super::super::Terminator;
    use super::Edge;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

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

    #[test]
    #[should_panic(expected = "case should lower to a Bool branch")]
    fn bool_branch_edge_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            bool_branch_edges(
                plan.int_function(IntFunctionId(0))
                    .body()
                    .block_graph()
                    .blocks()
                    .next()
                    .unwrap()
                    .terminator(),
            );
        });
    }

    fn bool_branch_edges(terminator: &Terminator) -> (&Edge, &Edge) {
        let Terminator::BoolBranch(branch) = terminator else {
            panic!("case should lower to a Bool branch");
        };
        (branch.true_(), branch.false_())
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let terminator = plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .blocks()
                .next()
                .unwrap()
                .terminator();
            let (true_, false_) = bool_branch_edges(terminator);
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(true_);
            context.push(' ');
            context.write(false_);
        });
    }
}

#[cfg(test)]
mod match_edge_explain_tests {
    use super::super::Terminator;
    use super::MatchEdge;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

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

    #[test]
    #[should_panic(expected = "let assert should lower to a match terminator")]
    fn match_edge_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            match_success_edge(
                plan.int_function(IntFunctionId(0))
                    .body()
                    .block_graph()
                    .blocks()
                    .next()
                    .unwrap()
                    .terminator(),
            );
        });
    }

    fn match_success_edge(terminator: &Terminator) -> &MatchEdge {
        let Terminator::Match(matcher) = terminator else {
            panic!("let assert should lower to a match terminator");
        };
        matcher.success()
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let terminator = plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .blocks()
                .next()
                .unwrap()
                .terminator();
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(match_success_edge(terminator));
        });
    }
}
