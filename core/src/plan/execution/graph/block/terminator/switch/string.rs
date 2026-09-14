use super::super::Edge;
use crate::plan::Text;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::StringLocalId;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub struct StringSwitch {
    pub subject: StringLocalId,
    pub clauses: Table<(Text, Edge)>,
    pub fallback: Edge,
}

impl StringSwitch {
    pub(in crate::plan::execution) fn new(
        subject: StringLocalId,
        clauses: Table<(Text, Edge)>,
        fallback: Edge,
    ) -> Self {
        Self {
            subject,
            clauses,
            fallback,
        }
    }

    pub(crate) fn subject(&self) -> StringLocalId {
        self.subject
    }

    pub(crate) fn clauses(&self) -> &[(Text, Edge)] {
        &self.clauses
    }

    pub(crate) fn fallback(&self) -> &Edge {
        &self.fallback
    }
}

impl Explain for StringSwitch {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("switch.string ");
        context.write(&self.subject());
        super::write_clauses(context, self.clauses(), |context, pattern| {
            context.push_str(&format!("{pattern:?}"));
        });
        context.push_str(" fallback=");
        context.write(self.fallback());
    }
}

impl Emit for StringSwitch {
    fn emit(&self, output: &mut Rust) {
        let Self {
            subject,
            clauses,
            fallback,
        } = self;
        output.structure(
            "graph::StringSwitch",
            &[
                ("subject", subject),
                ("clauses", clauses),
                ("fallback", fallback),
            ],
        );
    }
}

#[cfg(test)]
mod emission_tests {
    use super::StringSwitch;
    use crate::plan::execution::graph::{BlockId, Edge, IntLocalId, ParamLocal, StringLocalId};
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn emits_switch_string_with_edge_arguments() {
        let value = StringSwitch::new(
            StringLocalId(2),
            vec![(
                "key".into(),
                Edge::new(BlockId(3), vec![ParamLocal::Int(IntLocalId(5))]),
            )]
            .into(),
            Edge::new(BlockId(4), Vec::new()),
        );
        assert_eq!(
            Rust::expression(&value),
            r#"
data::graph::StringSwitch {
    subject: data::graph::StringLocalId(2),
    clauses: data::Storage::Static(&[
        (data::Text::Static("key"), data::graph::Edge {
            target: data::graph::BlockId(3),
            args: data::Storage::Static(&[
                data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
            ]),
        }),
    ]),
    fallback: data::graph::Edge {
        target: data::graph::BlockId(4),
        args: data::Storage::Static(&[]),
    },
}"#
            .trim_start_matches('\n')
        );
    }
}

#[cfg(test)]
mod explain_tests {
    use super::super::super::Terminator;
    use super::StringSwitch;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_string_switch() {
        let source = r#"
fn identity(value: String) { value }
pub fn main() { case identity("one") { "one" -> 1 _ -> 0 } }
"#;
        let expected = "switch.string %string#1 clauses=[\"one\"->b1()] fallback=b2()";

        assert_explanation(source, expected);
    }

    #[test]
    #[should_panic(expected = "source should lower one String switch")]
    fn string_switch_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            string_switch(&terminators(plan));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower one String switch")]
    fn string_switch_uniqueness_guard_is_visible() {
        let source = r#"
fn identity(value: String) { value }
pub fn main() { case identity("one") { "one" -> 1 _ -> 0 } }
"#;
        explain::with_execution_plan(source, |plan| {
            let switch = string_switch(&terminators(plan));
            string_switch_from_nodes(&[switch, switch]);
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

    fn string_switch<'a>(terminators: &[&'a Terminator]) -> &'a StringSwitch {
        let switches = terminators
            .iter()
            .copied()
            .filter_map(|terminator| match terminator {
                Terminator::StringSwitch(switch) => Some(switch),
                _ => None,
            })
            .collect::<Vec<_>>();
        string_switch_from_nodes(&switches)
    }

    fn string_switch_from_nodes<'a>(switches: &[&'a StringSwitch]) -> &'a StringSwitch {
        let [switch] = switches else {
            panic!("source should lower one String switch");
        };
        switch
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let switch = string_switch(&terminators(plan));
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(switch);
        });
    }
}
