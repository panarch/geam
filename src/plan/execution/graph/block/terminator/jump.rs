use super::Edge;

pub(crate) struct Jump {
    edge: Edge,
}

impl Jump {
    pub(in crate::plan::execution) fn new(edge: Edge) -> Self {
        Self { edge }
    }

    pub(crate) fn edge(&self) -> &Edge {
        &self.edge
    }
}

use crate::plan::execution::explain::{Explain, ExplainContext};

impl Explain for Jump {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("jump ");
        context.write(self.edge());
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::Terminator;
    use super::Jump;
    use crate::plan::execution::{IntFunctionId, explain};

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
    ) -> Vec<&crate::plan::execution::Terminator> {
        plan.int_function(IntFunctionId(0))
            .graph()
            .blocks()
            .iter()
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
