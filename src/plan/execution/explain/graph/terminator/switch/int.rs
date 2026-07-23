use super::super::super::super::value::ExplainLocal;
use super::super::super::edge::write_edge;
use crate::plan::execution::{Edge, IntLocalId};
use num_bigint::BigInt;

pub(in super::super) fn write_int_switch(
    output: &mut String,
    subject: &IntLocalId,
    clauses: &[(BigInt, Edge)],
    fallback: &Edge,
) {
    output.push_str("switch.int ");
    subject.write_local(output);
    super::write_clauses(output, clauses, |output, pattern| {
        output.push_str(&pattern.to_string());
    });
    output.push_str(" fallback=");
    write_edge(output, fallback);
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_int_switch() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() { case identity(1) { 1 -> 1 _ -> 0 } }
"#;
        let expected = "switch.int %int#1 clauses=[1->b1()] fallback=b2()";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let function = plan.int_function(IntFunctionId(0));
            let terminators = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            let (subject, clauses, fallback) = int_switch(&terminators);
            super::write_int_switch(output, subject, clauses, fallback);
        });
    }

    type IntSwitch<'a> = (
        &'a crate::plan::execution::IntLocalId,
        &'a [(num_bigint::BigInt, crate::plan::execution::Edge)],
        &'a crate::plan::execution::Edge,
    );

    fn int_switch<'a>(terminators: &[&'a Terminator]) -> IntSwitch<'a> {
        let mut switches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::IntSwitch {
                    subject,
                    clauses,
                    fallback,
                } => Some((subject, clauses.as_ref(), fallback)),
                _ => None,
            });
        let Some(switch) = switches.next() else {
            panic!("source should lower one Int switch");
        };
        if switches.next().is_some() {
            panic!("source should lower one Int switch");
        }
        switch
    }

    #[test]
    #[should_panic(expected = "source should lower one Int switch")]
    fn int_switch_shape_guard_is_visible() {
        super::super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            int_switch(&terminators);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one Int switch")]
    fn int_switch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() { case identity(1) { 1 -> 1 _ -> 0 } }
"#;
        super::super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::IntSwitch { .. }))
                .expect("source should lower an Int switch");
            int_switch(&[terminator, terminator]);
        });
    }
}
