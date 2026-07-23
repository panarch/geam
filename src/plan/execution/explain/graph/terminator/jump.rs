use super::super::edge::write_edge;
use crate::plan::execution::Edge;

pub(super) fn write_jump(output: &mut String, edge: &Edge) {
    output.push_str("jump ");
    write_edge(output, edge);
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_jump() {
        assert_explanation(
            r#"
fn identity(value: Bool) { value }

pub fn main() {
  let selected = case identity(True) {
    True -> 1
    False -> panic
  }
  selected + 3
}
"#,
            "jump b2(%int#0)",
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
            super::write_jump(output, jump(&terminators));
        });
    }

    fn jump<'a>(terminators: &[&'a Terminator]) -> &'a crate::plan::execution::Edge {
        let mut jumps = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::Jump(edge) => Some(edge),
                _ => None,
            });
        let Some(edge) = jumps.next() else {
            panic!("source should lower one jump");
        };
        if jumps.next().is_some() {
            panic!("source should lower one jump");
        }
        edge
    }

    #[test]
    #[should_panic(expected = "source should lower one jump")]
    fn jump_shape_guard_is_visible() {
        super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            jump(&terminators);
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
        super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::Jump(_)))
                .expect("source should lower a jump");
            jump(&[terminator, terminator]);
        });
    }
}
