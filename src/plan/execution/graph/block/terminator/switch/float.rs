use super::super::Edge;
use crate::plan::execution::FloatLocalId;
use crate::plan::execution::explain::{Explain, ExplainContext};

pub(crate) struct FloatSwitch {
    subject: FloatLocalId,
    clauses: Box<[(f64, Edge)]>,
    fallback: Edge,
}

impl FloatSwitch {
    pub(in crate::plan::execution) fn new(
        subject: FloatLocalId,
        clauses: Box<[(f64, Edge)]>,
        fallback: Edge,
    ) -> Self {
        Self {
            subject,
            clauses,
            fallback,
        }
    }

    pub(crate) fn subject(&self) -> FloatLocalId {
        self.subject
    }

    pub(crate) fn clauses(&self) -> &[(f64, Edge)] {
        &self.clauses
    }

    pub(crate) fn fallback(&self) -> &Edge {
        &self.fallback
    }
}

impl Explain for FloatSwitch {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("switch.float ");
        context.write(&self.subject());
        super::write_clauses(context, self.clauses(), |context, pattern| {
            context.push_str(&format!("{pattern:?}"));
        });
        context.push_str(" fallback=");
        context.write(self.fallback());
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::super::Terminator;
    use super::FloatSwitch;
    use crate::plan::execution::{IntFunctionId, explain};

    #[test]
    fn writes_float_switch() {
        let source = r#"
fn identity(value: Float) { value }
pub fn main() { case identity(1.0) { 1.0 -> 1 _ -> 0 } }
"#;
        let expected = "switch.float %float#1 clauses=[1.0->b1()] fallback=b2()";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one Float switch")]
    fn float_switch_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            float_switch(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one Float switch")]
    fn float_switch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: Float) { value }
pub fn main() { case identity(1.0) { 1.0 -> 1 _ -> 0 } }
"#;
        explain::with_execution_plan(source, |plan| {
            let switch = float_switch(&terminators(plan));
            float_switch_from_nodes(&[switch, switch]);
        });
    }

    fn terminators(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> Vec<&crate::plan::execution::Terminator> {
        plan.int_function(IntFunctionId(0))
            .body()
            .block_graph()
            .blocks()
            .iter()
            .map(|block| block.terminator())
            .collect()
    }

    fn float_switch<'a>(terminators: &[&'a Terminator]) -> &'a FloatSwitch {
        let switches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::FloatSwitch(switch) => Some(switch),
                _ => None,
            })
            .collect::<Vec<_>>();
        float_switch_from_nodes(&switches)
    }

    fn float_switch_from_nodes<'a>(switches: &[&'a FloatSwitch]) -> &'a FloatSwitch {
        let [switch] = switches else {
            panic!("source should lower one Float switch");
        };
        switch
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let switch = float_switch(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(switch);
        });
    }
}
