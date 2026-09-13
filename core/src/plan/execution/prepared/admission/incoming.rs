use super::block::Blocks;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BlockId, BoolLocalId, Edge, Match, MatchEdgeArgument, ParamLocal, StringLocalId, Terminator,
};

pub(super) struct Incoming<'data> {
    pub(super) block: BlockId,
    pub(super) value: Input<'data>,
    pub(super) condition: Condition<'data>,
}

pub(super) enum Input<'data> {
    Local(&'data ParamLocal),
    Binding { index: usize, matcher: &'data Match },
}

#[derive(Clone, Copy)]
pub(super) enum Condition<'data> {
    Always,
    Bool {
        subject: BoolLocalId,
        truth: bool,
    },
    Match {
        matcher: &'data Match,
        success: bool,
    },
    String {
        subject: StringLocalId,
        value: &'data str,
    },
}

pub(super) fn parameter<'data, Graph: ExecutionGraphProfile>(
    blocks: &Blocks<'data, Graph>,
    target: BlockId,
    index: usize,
) -> Option<Vec<Incoming<'data>>> {
    let mut inputs = Vec::new();
    for (parent, block) in blocks.iter().enumerate() {
        let parent = BlockId(parent);
        let mut regular = |edge: &'data Edge, condition| {
            if edge.target == target {
                inputs.push(Incoming {
                    block: parent,
                    value: Input::Local(edge.args.get(index)?),
                    condition,
                });
            }
            Some(())
        };
        match block.terminator() {
            Terminator::Jump(value) => regular(&value.edge, Condition::Always)?,
            Terminator::BoolBranch(value) => {
                regular(
                    &value.true_,
                    Condition::Bool {
                        subject: value.subject,
                        truth: true,
                    },
                )?;
                regular(
                    &value.false_,
                    Condition::Bool {
                        subject: value.subject,
                        truth: false,
                    },
                )?;
            }
            Terminator::IntSwitch(value) => {
                for (_, edge) in value.clauses.iter() {
                    regular(edge, Condition::Always)?;
                }
                regular(&value.fallback, Condition::Always)?;
            }
            Terminator::FloatSwitch(value) => {
                for (_, edge) in value.clauses.iter() {
                    regular(edge, Condition::Always)?;
                }
                regular(&value.fallback, Condition::Always)?;
            }
            Terminator::StringSwitch(value) => {
                for (literal, edge) in value.clauses.iter() {
                    regular(
                        edge,
                        Condition::String {
                            subject: value.subject,
                            value: literal.as_str(),
                        },
                    )?;
                }
                regular(&value.fallback, Condition::Always)?;
            }
            Terminator::Echo(value) => regular(&value.next, Condition::Always)?,
            Terminator::Match(value) => {
                regular(
                    &value.failure,
                    Condition::Match {
                        matcher: value,
                        success: false,
                    },
                )?;
                if value.success.target == target {
                    let argument = match value.success.args.get(index)? {
                        MatchEdgeArgument::Value(value) => Input::Local(value),
                        MatchEdgeArgument::Binding(index) => Input::Binding {
                            index: *index,
                            matcher: value,
                        },
                    };
                    inputs.push(Incoming {
                        block: parent,
                        value: argument,
                        condition: Condition::Match {
                            matcher: value,
                            success: true,
                        },
                    });
                }
            }
            Terminator::Exit(_)
            | Terminator::SourceStop(_)
            | Terminator::LetAssertPanic(_)
            | Terminator::NeverCall(_) => {}
        }
    }
    Some(inputs)
}

#[cfg(test)]
mod tests {
    use super::{Blocks, Condition, Input, parameter};
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockHeader, BlockId, BoolBranch, BoolLocalId, Echo, Edge, FloatLocalId,
        FloatSwitch, IntLocalId, IntSwitch, Jump, Match, MatchEdge, MatchEdgeArgument,
        MatchPattern, MatchPatternBinding, ParamLocal, ParamSlot, ProfiledBlockGraph,
        StringLocalId, StringSwitch, Terminator,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::ValueShapeId;
    use crate::plan::{EchoSite, SourceSpan};
    use std::convert::Infallible;

    #[test]
    fn regular_edges_retain_branch_conditions_and_source_order() {
        let first = Edge::new(BlockId(1), vec![ParamLocal::Int(IntLocalId(0))]);
        for (kind, terminator) in regular_terminators(first.clone(), first) {
            let graph = graph(terminator);
            let blocks = Blocks::admit(&graph).unwrap();
            let incoming = parameter(&blocks, BlockId(1), 0).unwrap();
            assert_eq!(
                incoming.len(),
                if kind == "jump" || kind == "echo" {
                    1
                } else {
                    2
                }
            );
            for value in &incoming {
                assert_eq!(value.block, BlockId(0));
                assert!(matches!(value.value, Input::Local(local)
                    if local == &ParamLocal::Int(IntLocalId(0))));
            }
            match kind {
                "bool" => {
                    assert!(matches!(
                        incoming[0].condition,
                        Condition::Bool { subject, truth }
                            if subject == BoolLocalId(0) && truth
                    ));
                    assert!(matches!(
                        incoming[1].condition,
                        Condition::Bool { subject, truth }
                            if subject == BoolLocalId(0) && !truth
                    ));
                }
                "string" => {
                    let strings = incoming
                        .iter()
                        .filter_map(|input| match input.condition {
                            Condition::String { subject, value } => Some((subject, value)),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(strings, vec![(StringLocalId(0), "selected")]);
                    assert_eq!(
                        std::mem::discriminant(&incoming[1].condition),
                        std::mem::discriminant(&Condition::Always),
                    );
                }
                _ => assert!(
                    incoming
                        .iter()
                        .all(|input| std::mem::discriminant(&input.condition)
                            == std::mem::discriminant(&Condition::Always))
                ),
            }
            assert!(parameter(&blocks, BlockId(2), 0).unwrap().is_empty());
        }
    }

    #[test]
    fn any_missing_regular_argument_prevents_partial_incoming_evidence() {
        let present = Edge::new(BlockId(1), vec![ParamLocal::Int(IntLocalId(0))]);
        let absent = Edge::new(BlockId(1), Vec::new());
        for (first, second) in [(absent.clone(), present.clone()), (present, absent)] {
            for (kind, terminator) in regular_terminators(first.clone(), second.clone()) {
                let graph = graph(terminator);
                let blocks = Blocks::admit(&graph).unwrap();
                let incoming = parameter(&blocks, BlockId(1), 0);
                let has_single_edge = kind == "jump" || kind == "echo";
                assert_eq!(
                    incoming.is_none(),
                    !has_single_edge || first.args.is_empty()
                );
                assert!(parameter(&blocks, BlockId(2), 3).unwrap().is_empty());
            }
        }
    }

    #[test]
    fn match_edges_distinguish_bindings_local_arguments_and_failure() {
        for argument in [
            MatchEdgeArgument::Binding(0),
            MatchEdgeArgument::Value(ParamLocal::Int(IntLocalId(0))),
        ] {
            let graph = graph(Terminator::Match(Match {
                subject: ParamLocal::Int(IntLocalId(0)),
                pattern: MatchPattern::Bind(MatchPatternBinding::new(0)),
                success: MatchEdge::new(BlockId(1), vec![argument]),
                failure: Edge::new(BlockId(1), vec![ParamLocal::Int(IntLocalId(0))]),
            }));
            let blocks = Blocks::admit(&graph).unwrap();
            let incoming = parameter(&blocks, BlockId(1), 0).unwrap();
            assert_eq!(incoming.len(), 2);
            let matchers = graph
                .blocks()
                .filter_map(|block| match block.terminator() {
                    Terminator::Match(value) => Some(value),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(matchers.len(), 1);
            let original = matchers[0];
            assert_eq!(incoming[0].block, BlockId(0));
            assert!(matches!(incoming[0].value, Input::Local(value)
                if value == &original.failure.args[0]));
            assert!(
                matches!(incoming[0].condition, Condition::Match { matcher, success: false }
                if std::ptr::eq(matcher, original))
            );
            assert_eq!(incoming[1].block, BlockId(0));
            assert!(
                matches!(incoming[1].condition, Condition::Match { matcher, success: true }
                if std::ptr::eq(matcher, original))
            );
            match &original.success.args[0] {
                MatchEdgeArgument::Binding(index) => {
                    assert!(
                        matches!(incoming[1].value, Input::Binding { index: actual, matcher }
                        if actual == *index && std::ptr::eq(matcher, original))
                    );
                }
                MatchEdgeArgument::Value(expected) => {
                    assert!(matches!(incoming[1].value, Input::Local(actual)
                        if std::ptr::eq(actual, expected)));
                }
            }
            assert!(parameter(&blocks, BlockId(1), 1).is_none());
            assert!(parameter(&blocks, BlockId(2), 0).unwrap().is_empty());
        }
        for (success, failure) in [
            (
                MatchEdge::new(BlockId(1), Vec::new()),
                Edge::new(BlockId(2), Vec::new()),
            ),
            (
                MatchEdge::new(BlockId(2), Vec::new()),
                Edge::new(BlockId(1), Vec::new()),
            ),
        ] {
            let graph = graph(Terminator::Match(Match {
                subject: ParamLocal::Int(IntLocalId(0)),
                pattern: MatchPattern::Bind(MatchPatternBinding::new(0)),
                success,
                failure,
            }));
            let blocks = Blocks::admit(&graph).unwrap();
            assert!(parameter(&blocks, BlockId(1), 0).is_none());
        }
    }

    fn regular_terminators(first: Edge, second: Edge) -> Vec<(&'static str, Terminator)> {
        vec![
            (
                "jump",
                Terminator::Jump(Jump {
                    edge: first.clone(),
                }),
            ),
            (
                "bool",
                Terminator::BoolBranch(BoolBranch {
                    subject: BoolLocalId(0),
                    true_: first.clone(),
                    false_: second.clone(),
                }),
            ),
            (
                "int",
                Terminator::IntSwitch(IntSwitch {
                    subject: IntLocalId(0),
                    clauses: vec![(num_bigint::BigInt::from(42).into(), first.clone())].into(),
                    fallback: second.clone(),
                }),
            ),
            (
                "float",
                Terminator::FloatSwitch(FloatSwitch {
                    subject: FloatLocalId(0),
                    clauses: vec![(1.5, first.clone())].into(),
                    fallback: second.clone(),
                }),
            ),
            (
                "string",
                Terminator::StringSwitch(StringSwitch {
                    subject: StringLocalId(0),
                    clauses: vec![("selected".into(), first.clone())].into(),
                    fallback: second,
                }),
            ),
            (
                "echo",
                Terminator::Echo(Echo {
                    subject: ParamLocal::Int(IntLocalId(0)),
                    message: None,
                    site: EchoSite::new("example".into(), "main".into(), SourceSpan::new(0, 1)),
                    next: first,
                }),
            ),
        ]
    }

    fn graph(terminator: Terminator) -> ProfiledBlockGraph<Infallible> {
        ProfiledBlockGraph {
            entry: BlockId(0),
            blocks: vec![
                BlockHeader {
                    params: 0..4,
                    instructions: 0..0,
                    terminator,
                },
                BlockHeader {
                    params: 4..5,
                    instructions: 0..0,
                    terminator: Terminator::Exit(BlockGraphExitId(0)),
                },
                BlockHeader {
                    params: 5..5,
                    instructions: 0..0,
                    terminator: Terminator::Exit(BlockGraphExitId(1)),
                },
            ]
            .into(),
            params: vec![
                ParamSlot::new(ParamLocal::Int(IntLocalId(0)), ValueShapeId(0)),
                ParamSlot::new(ParamLocal::Float(FloatLocalId(0)), ValueShapeId(1)),
                ParamSlot::new(ParamLocal::Bool(BoolLocalId(0)), ValueShapeId(2)),
                ParamSlot::new(ParamLocal::String(StringLocalId(0)), ValueShapeId(3)),
                ParamSlot::new(ParamLocal::Int(IntLocalId(0)), ValueShapeId(0)),
            ]
            .into(),
            instructions: Table::Static(&[]),
        }
    }
}
