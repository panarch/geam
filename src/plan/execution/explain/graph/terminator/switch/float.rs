use super::super::super::super::value::ExplainLocal;
use super::super::super::edge::write_edge;
use crate::plan::execution::{Edge, FloatLocalId};

pub(in super::super) fn write_float_switch(
    output: &mut String,
    subject: &FloatLocalId,
    clauses: &[(f64, Edge)],
    fallback: &Edge,
) {
    output.push_str("switch.float ");
    subject.write_local(output);
    super::write_clauses(output, clauses, |output, pattern| {
        output.push_str(&format!("{pattern:?}"));
    });
    output.push_str(" fallback=");
    write_edge(output, fallback);
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{IntFunctionId, Terminator};

    #[test]
    fn writes_float_switch() {
        assert_explanation(
            r#"
fn identity(value: Float) { value }
pub fn main() { case identity(1.0) { 1.0 -> 1 _ -> 0 } }
"#,
            "switch.float %float#1 clauses=[1.0->b1()] fallback=b2()",
        );
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
            let (subject, clauses, fallback) = float_switch(&terminators);
            super::write_float_switch(output, subject, clauses, fallback);
        });
    }

    type FloatSwitch<'a> = (
        &'a crate::plan::execution::FloatLocalId,
        &'a [(f64, crate::plan::execution::Edge)],
        &'a crate::plan::execution::Edge,
    );

    fn float_switch<'a>(terminators: &[&'a Terminator]) -> FloatSwitch<'a> {
        let mut switches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::FloatSwitch {
                    subject,
                    clauses,
                    fallback,
                } => Some((subject, clauses.as_ref(), fallback)),
                _ => None,
            });
        let Some(switch) = switches.next() else {
            panic!("source should lower one Float switch");
        };
        if switches.next().is_some() {
            panic!("source should lower one Float switch");
        }
        switch
    }

    #[test]
    #[should_panic(expected = "source should lower one Float switch")]
    fn float_switch_shape_guard_is_visible() {
        super::super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            float_switch(&terminators);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one Float switch")]
    fn float_switch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: Float) { value }
pub fn main() { case identity(1.0) { 1.0 -> 1 _ -> 0 } }
"#;
        super::super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::FloatSwitch { .. }))
                .expect("source should lower a Float switch");
            float_switch(&[terminator, terminator]);
        });
    }
}
