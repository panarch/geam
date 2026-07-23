use super::super::super::super::value::ExplainLocal;
use super::super::super::edge::write_edge;
use crate::plan::execution::{Edge, StringLocalId};
use ecow::EcoString;

pub(in super::super) fn write_string_switch(
    output: &mut String,
    subject: &StringLocalId,
    clauses: &[(EcoString, Edge)],
    fallback: &Edge,
) {
    output.push_str("switch.string ");
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
    fn writes_string_switch() {
        let source = r#"
fn identity(value: String) { value }
pub fn main() { case identity("one") { "one" -> 1 _ -> 0 } }
"#;
        let expected = "switch.string %string#1 clauses=[\"one\"->b1()] fallback=b2()";

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
            let (subject, clauses, fallback) = string_switch(&terminators);
            super::write_string_switch(output, &subject, clauses, fallback);
        });
    }

    type StringSwitch<'a> = (
        crate::plan::execution::StringLocalId,
        &'a [(ecow::EcoString, crate::plan::execution::Edge)],
        &'a crate::plan::execution::Edge,
    );

    fn string_switch<'a>(terminators: &[&'a Terminator]) -> StringSwitch<'a> {
        let mut switches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::StringSwitch(switch) => {
                    Some((switch.subject(), switch.clauses(), switch.fallback()))
                }
                _ => None,
            });
        let Some(switch) = switches.next() else {
            panic!("source should lower one String switch");
        };
        if switches.next().is_some() {
            panic!("source should lower one String switch");
        }
        switch
    }

    #[test]
    #[should_panic(expected = "source should lower one String switch")]
    fn string_switch_shape_guard_is_visible() {
        super::super::super::super::super::with_execution_plan("pub fn main() { 1 }", |plan| {
            let terminators = plan
                .int_function(IntFunctionId(0))
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .collect::<Vec<_>>();
            string_switch(&terminators);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one String switch")]
    fn string_switch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: String) { value }
pub fn main() { case identity("one") { "one" -> 1 _ -> 0 } }
"#;
        super::super::super::super::super::with_execution_plan(source, |plan| {
            let function = plan.int_function(IntFunctionId(0));
            let terminator = function
                .graph()
                .blocks()
                .iter()
                .map(|block| block.terminator())
                .find(|terminator| matches!(terminator, Terminator::StringSwitch(_)))
                .expect("source should lower a String switch");
            string_switch(&[terminator, terminator]);
        });
    }
}
