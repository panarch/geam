use super::Edge;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::prepared::rust::{Emit, Rust};

#[derive(Clone)]
pub struct Jump {
    pub edge: Edge,
}

impl Jump {
    pub(in crate::plan::execution) fn new(edge: Edge) -> Self {
        Self { edge }
    }

    pub(crate) fn edge(&self) -> &Edge {
        &self.edge
    }
}

impl Explain for Jump {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("jump ");
        context.write(self.edge());
    }
}

impl Emit for Jump {
    fn emit(&self, output: &mut Rust) {
        let Self { edge } = self;
        output.structure("graph::Jump", &[("edge", edge)]);
    }
}

#[cfg(test)]
mod emission_tests {
    use super::Jump;
    use crate::plan::execution::graph::{BlockId, Edge, IntLocalId, ParamLocal};
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn emits_jump_with_edge_arguments() {
        let value = Jump::new(Edge::new(BlockId(3), vec![ParamLocal::Int(IntLocalId(5))]));
        assert_eq!(
            Rust::expression(&value),
            r#"
data::graph::Jump {
    edge: data::graph::Edge {
        target: data::graph::BlockId(3),
        args: data::Storage::Static(&[
            data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
        ]),
    },
}"#
            .trim_start_matches('\n')
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::Jump;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_jump() {
        let source = r#"
fn identity(value: Bool) { value }

pub fn main() {
  let selected = case identity(True) {
    True -> 1
    False -> panic
  }
  selected + 3
}
"#;
        let expected = "jump b2(%int#0)";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one jump")]
    fn jump_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            jump(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one jump")]
    fn jump_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: Bool) { value }

pub fn main() {
  let selected = case identity(True) {
    True -> 1
    False -> panic
  }
  selected + 3
}
"#;
        explain::with_execution_plan(source, |plan| {
            let jump = jump(&terminators(plan));
            jump_from_nodes(&[jump, jump]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::graph::Terminator> {
        plan.int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .blocks()
            .map(|block| block.terminator())
            .collect()
    }

    fn jump<'a>(terminators: &[&'a Terminator]) -> &'a Jump {
        let jumps = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Jump(jump) => Some(jump),
                _ => None,
            })
            .collect::<Vec<_>>();
        jump_from_nodes(&jumps)
    }

    fn jump_from_nodes<'a>(jumps: &[&'a Jump]) -> &'a Jump {
        let [jump] = jumps else {
            panic!("source should lower one jump");
        };
        jump
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let jump = jump(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(jump);
        });
    }
}
