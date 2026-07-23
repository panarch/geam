use super::super::Edge;
use crate::plan::execution::IntLocalId;
use num_bigint::BigInt;

pub(crate) struct IntSwitch {
    subject: IntLocalId,
    clauses: Box<[(BigInt, Edge)]>,
    fallback: Edge,
}

impl IntSwitch {
    pub(in crate::plan::execution) fn new(
        subject: IntLocalId,
        clauses: Box<[(BigInt, Edge)]>,
        fallback: Edge,
    ) -> Self {
        Self {
            subject,
            clauses,
            fallback,
        }
    }

    pub(crate) fn subject(&self) -> IntLocalId {
        self.subject
    }

    pub(crate) fn clauses(&self) -> &[(BigInt, Edge)] {
        &self.clauses
    }

    pub(crate) fn fallback(&self) -> &Edge {
        &self.fallback
    }
}

use crate::plan::execution::explain::{Explain, ExplainContext};

impl Explain for IntSwitch {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("switch.int ");
        context.write(&self.subject());
        super::write_clauses(context, self.clauses(), |context, pattern| {
            context.push_str(&pattern.to_string());
        });
        context.push_str(" fallback=");
        context.write(self.fallback());
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::super::Terminator;
    use super::IntSwitch;
    use crate::plan::execution::{IntFunctionId, explain};

    #[test]
    fn writes_int_switch() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() { case identity(1) { 1 -> 1 _ -> 0 } }
"#;
        let expected = "switch.int %int#1 clauses=[1->b1()] fallback=b2()";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one Int switch")]
    fn int_switch_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            int_switch(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one Int switch")]
    fn int_switch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() { case identity(1) { 1 -> 1 _ -> 0 } }
"#;
        explain::with_execution_plan(source, |plan| {
            let switch = int_switch(&terminators(plan));
            int_switch_from_nodes(&[switch, switch]);
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

    fn int_switch<'a>(terminators: &[&'a Terminator]) -> &'a IntSwitch {
        let switches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::IntSwitch(switch) => Some(switch),
                _ => None,
            })
            .collect::<Vec<_>>();
        int_switch_from_nodes(&switches)
    }

    fn int_switch_from_nodes<'a>(switches: &[&'a IntSwitch]) -> &'a IntSwitch {
        let [switch] = switches else {
            panic!("source should lower one Int switch");
        };
        switch
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let switch = int_switch(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(switch);
        });
    }
}
