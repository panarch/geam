use super::super::BlockId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{ParamLocal, Transfer};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub struct Edge {
    pub target: BlockId,
    pub args: Table<ParamLocal>,
    pub transfer: Transfer,
}

#[derive(Clone)]
pub struct MatchEdge {
    pub target: BlockId,
    pub args: Table<MatchEdgeArgument>,
    pub bindings: Table<usize>,
    pub transfer: Transfer,
}

#[derive(Clone)]
pub enum MatchEdgeArgument {
    Binding(usize),
    Value(ParamLocal),
}

impl Edge {
    pub(in crate::plan::execution) fn new(
        target: BlockId,
        args: Vec<ParamLocal>,
        transfer: Transfer,
    ) -> Self {
        Self {
            target,
            args: args.into(),
            transfer,
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
    pub(in crate::plan::execution) fn new(
        target: BlockId,
        args: Vec<MatchEdgeArgument>,
        bindings: Vec<usize>,
        transfer: Transfer,
    ) -> Self {
        Self {
            target,
            args: args.into(),
            bindings: bindings.into(),
            transfer,
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
        let Self {
            target,
            args,
            transfer,
        } = self;
        output.structure(
            "graph::Edge",
            &[("target", target), ("args", args), ("transfer", transfer)],
        );
    }
}

impl Emit for MatchEdge {
    fn emit(&self, output: &mut Rust) {
        let Self {
            target,
            args,
            bindings,
            transfer,
        } = self;
        output.structure(
            "graph::MatchEdge",
            &[
                ("target", target),
                ("args", args),
                ("bindings", bindings),
                ("transfer", transfer),
            ],
        );
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
mod emission_tests {
    use super::{BlockId, Edge, MatchEdge, MatchEdgeArgument, ParamLocal, Transfer};
    use crate::plan::execution::graph::{FamilyTransfer, IntLocalId, StorageFamily, TransferStep};
    use crate::plan::execution::prepared::rust::Rust;

    #[test]
    fn emits_regular_edge_with_its_exact_transfer() {
        let edge = Edge::new(
            BlockId(3),
            vec![ParamLocal::Int(IntLocalId(2))],
            Transfer {
                families: vec![FamilyTransfer {
                    family: StorageFamily::Int,
                    length: 1,
                    steps: vec![TransferStep {
                        source: 2,
                        destination: 0,
                    }]
                    .into(),
                }]
                .into(),
            },
        );
        assert_eq!(
            Rust::expression(&edge),
            r#"
data::graph::Edge {
    target: data::graph::BlockId(3),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(2)),
    ]),
    transfer: data::graph::Transfer {
        families: data::Storage::Static(&[
            data::graph::FamilyTransfer {
                family: data::graph::StorageFamily::Int,
                length: 1,
                steps: data::Storage::Static(&[
                    data::graph::TransferStep {
                        source: 2,
                        destination: 0,
                    },
                ]),
            },
        ]),
    },
}"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn emits_match_edge_with_selected_bindings_and_transfer_order() {
        let edge = MatchEdge::new(
            BlockId(3),
            vec![
                MatchEdgeArgument::Binding(4),
                MatchEdgeArgument::Value(ParamLocal::Int(IntLocalId(0))),
            ],
            vec![4],
            Transfer {
                families: vec![FamilyTransfer {
                    family: StorageFamily::Int,
                    length: 2,
                    steps: vec![TransferStep {
                        source: 1,
                        destination: 0,
                    }]
                    .into(),
                }]
                .into(),
            },
        );
        assert_eq!(Rust::expression(&edge), r#"
data::graph::MatchEdge {
    target: data::graph::BlockId(3),
    args: data::Storage::Static(&[
        data::graph::MatchEdgeArgument::Binding(4),
        data::graph::MatchEdgeArgument::Value(data::graph::ParamLocal::Int(data::graph::IntLocalId(0))),
    ]),
    bindings: data::Storage::Static(&[
        4,
    ]),
    transfer: data::graph::Transfer {
        families: data::Storage::Static(&[
            data::graph::FamilyTransfer {
                family: data::graph::StorageFamily::Int,
                length: 2,
                steps: data::Storage::Static(&[
                    data::graph::TransferStep {
                        source: 1,
                        destination: 0,
                    },
                ]),
            },
        ]),
    },
}"#.trim_start_matches('\n'));
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
