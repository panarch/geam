mod access;
mod origin;
mod pattern;

use super::block::Blocks;
use super::incoming::{self, Condition, Input};
use super::local::Address;
use super::place::{self, Place, Projection};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BlockId, BoolInstruction, BoolLocalId, ParamLocal, ProfiledInstruction, ProfiledInstructionKind,
};
use origin::Origin;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

pub(super) struct Guards<'graph, 'data, Graph: ExecutionGraphProfile> {
    pub(super) blocks: &'graph Blocks<'data, Graph>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum GuardError {
    LengthOverflow,
    Unproved { local: Address, requirement: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Requirement<'data> {
    Length(usize),
    Prefix(Cow<'data, str>),
}

#[derive(Debug, PartialEq, Eq)]
struct Access<'data> {
    local: Address,
    requirement: Requirement<'data>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Query<'data> {
    block: BlockId,
    place: Place,
    requirement: Requirement<'data>,
}

#[derive(Debug, PartialEq, Eq)]
enum Visit<'data> {
    Enter(Query<'data>),
    Leave(Query<'data>),
}

impl<'data, Graph: ExecutionGraphProfile> Guards<'_, 'data, Graph> {
    pub(super) fn contradicts(&self, block_id: BlockId, condition: Condition<'data>) -> bool {
        let Condition::Bool {
            mut subject,
            mut truth,
        } = condition
        else {
            return false;
        };
        let Ok(block) = self.blocks.block(block_id) else {
            return false;
        };
        let mut visited = HashSet::new();
        loop {
            if !visited.insert(subject.0) {
                return false;
            }
            let Some(instruction) = block.instructions().iter().find(|instruction| {
                Address::of(&instruction.output.local) == Address::from(subject)
            }) else {
                return false;
            };
            let ProfiledInstructionKind::Bool(value) = &instruction.kind else {
                return false;
            };
            let (local, requirement) = match value {
                BoolInstruction::Not(value) => {
                    subject = *value;
                    truth = !truth;
                    continue;
                }
                BoolInstruction::Value(value) => return *value != truth,
                BoolInstruction::ListLengthAtLeast { value, length } if !truth => (
                    Address::of(&ParamLocal::List(value.clone())),
                    Requirement::Length(*length),
                ),
                BoolInstruction::ListLengthEquals { value, length } if truth => {
                    let Some(length) = length.checked_add(1) else {
                        return false;
                    };
                    (
                        Address::of(&ParamLocal::List(value.clone())),
                        Requirement::Length(length),
                    )
                }
                BoolInstruction::StringStartsWith { value, prefix } if !truth => (
                    Address::from(*value),
                    Requirement::Prefix(prefix.as_str().into()),
                ),
                _ => return false,
            };
            return self.proves(Query {
                block: block_id,
                place: Place::local(local),
                requirement,
            });
        }
    }

    pub(super) fn check(
        &self,
        block: BlockId,
        instruction: &'data ProfiledInstruction<Graph>,
    ) -> Result<(), GuardError> {
        if let Some(access) = access::instruction(&instruction.kind)?
            && !self.proves(Query {
                block,
                place: Place::local(access.local),
                requirement: access.requirement.clone(),
            })
        {
            return Err(GuardError::Unproved {
                local: access.local,
                requirement: access.requirement.description(),
            });
        }
        Ok(())
    }

    // Backward proofs cover all incoming paths. A cycle can preserve or weaken
    // an obligation, but cannot justify a progressively stronger one.
    fn proves(&self, query: Query<'data>) -> bool {
        let mut pending = vec![Visit::Enter(query)];
        let mut active =
            HashMap::<(BlockId, Address), (Vec<Projection>, Requirement<'data>)>::new();
        let mut complete = HashSet::new();
        while let Some(visit) = pending.pop() {
            let (query, block) = match visit {
                Visit::Enter(mut query) => {
                    let Ok(block) = self.blocks.block(query.block) else {
                        return false;
                    };
                    let Some(place) = query.place.in_block(block) else {
                        return false;
                    };
                    query.place = place;
                    (query, block)
                }
                Visit::Leave(query) => {
                    active.remove(&(query.block, query.place.root));
                    complete.insert(query);
                    continue;
                }
            };
            if query.requirement.is_empty() || complete.contains(&query) {
                continue;
            }
            let key = (query.block, query.place.root);
            if let Some((path, existing)) = active.get(&key) {
                if path == &query.place.path && existing.covers(&query.requirement) {
                    continue;
                }
                return false;
            }
            active.insert(key, (query.place.path.clone(), query.requirement.clone()));
            pending.push(Visit::Leave(query.clone()));
            if let Some(index) = block
                .params()
                .iter()
                .position(|slot| Address::of(&slot.local) == query.place.root)
            {
                if query.block == self.blocks.entry() {
                    return false;
                }
                let Some(inputs) = incoming::parameter(self.blocks, query.block, index) else {
                    return false;
                };
                for input in inputs {
                    match input.value {
                        Input::Local(source) => {
                            let Some(source) = query
                                .place
                                .with_root(Address::of(source))
                                .normalize(input.block, self.blocks)
                            else {
                                return false;
                            };
                            if self.condition(
                                input.block,
                                input.condition,
                                &source,
                                &query.requirement,
                            ) {
                                continue;
                            }
                            pending.push(Visit::Enter(Query {
                                block: input.block,
                                place: source,
                                requirement: query.requirement.clone(),
                            }));
                        }
                        Input::Binding { index, matcher } => {
                            match pattern::binding(
                                &matcher.pattern,
                                index,
                                &query.place.path,
                                &query.requirement,
                            ) {
                                pattern::BindingProof::Proven => {}
                                pattern::BindingProof::Source { path, requirement } => pending
                                    .push(Visit::Enter(Query {
                                        block: input.block,
                                        place: Place {
                                            root: Address::of(&matcher.subject),
                                            path,
                                        },
                                        requirement,
                                    })),
                                pattern::BindingProof::Unknown => return false,
                            }
                        }
                    }
                }
            } else if let Some(instruction) = block
                .instructions()
                .iter()
                .find(|instruction| Address::of(&instruction.output.local) == query.place.root)
            {
                if !query.place.path.is_empty()
                    || !self.origin(query, origin::instruction(&instruction.kind), &mut pending)
                {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn condition(
        &self,
        block: BlockId,
        condition: Condition<'data>,
        source: &Place,
        required: &Requirement<'data>,
    ) -> bool {
        match condition {
            Condition::Always => false,
            Condition::String { subject, value } => {
                self.same_place(block, Address::from(subject), source)
                    && required.accepts_text(value)
            }
            Condition::Match { matcher, success } => {
                let Some(subject) =
                    Place::local(Address::of(&matcher.subject)).normalize(block, self.blocks)
                else {
                    return false;
                };
                if subject.root != source.root || !source.path.starts_with(&subject.path) {
                    return false;
                }
                let path = &source.path[subject.path.len()..];
                if !success && !path.is_empty() {
                    return false;
                }
                place::pattern_at(&matcher.pattern, path)
                    .is_some_and(|pattern| pattern::establishes(pattern, success, required))
            }
            Condition::Bool { subject, truth } => {
                self.boolean(block, subject, truth, source, required)
            }
        }
    }

    fn boolean(
        &self,
        block: BlockId,
        mut subject: BoolLocalId,
        mut truth: bool,
        source: &Place,
        required: &Requirement<'data>,
    ) -> bool {
        let block_id = block;
        let Ok(block) = self.blocks.block(block_id) else {
            return false;
        };
        let mut visited = HashSet::new();
        loop {
            if !visited.insert(subject.0) {
                return false;
            }
            let Some(instruction) = block.instructions().iter().find(|instruction| {
                Address::of(&instruction.output.local) == Address::from(subject)
            }) else {
                return false;
            };
            let ProfiledInstructionKind::Bool(value) = &instruction.kind else {
                return false;
            };
            match value {
                BoolInstruction::Not(value) => {
                    subject = *value;
                    truth = !truth;
                }
                BoolInstruction::StringStartsWith { value, prefix } => {
                    return truth
                        && self.same_place(block_id, Address::from(*value), source)
                        && required.accepts_text(prefix.as_str());
                }
                BoolInstruction::ListLengthEquals { value, length }
                | BoolInstruction::ListLengthAtLeast { value, length } => {
                    let Requirement::Length(required) = required else {
                        return false;
                    };
                    if !self.same_place(
                        block_id,
                        Address::of(&ParamLocal::List(value.clone())),
                        source,
                    ) {
                        return false;
                    }
                    return if truth {
                        length >= required
                    } else {
                        matches!(
                            &instruction.kind,
                            ProfiledInstructionKind::Bool(BoolInstruction::ListLengthEquals { .. })
                        ) && *length == 0
                            && *required <= 1
                    };
                }
                _ => return false,
            }
        }
    }

    fn origin(
        &self,
        query: Query<'data>,
        origin: Origin<'data>,
        pending: &mut Vec<Visit<'data>>,
    ) -> bool {
        let (local, requirement) = match (origin, &query.requirement) {
            (Origin::List { head, tail }, Requirement::Length(length)) => {
                if head >= *length {
                    return true;
                }
                let Some(tail) = tail else {
                    return false;
                };
                (tail, Requirement::Length(length - head))
            }
            (Origin::ListDrop { source, count }, Requirement::Length(length)) => {
                let Some(length) = length.checked_add(count) else {
                    return false;
                };
                (source, Requirement::Length(length))
            }
            (Origin::Text(value), required) => return required.accepts_text(value),
            (Origin::TextDrop { source, prefix }, Requirement::Prefix(required)) => (
                source,
                Requirement::Prefix(format!("{prefix}{required}").into()),
            ),
            (Origin::Concatenate { left, right }, Requirement::Prefix(required)) => {
                let Ok(block) = self.blocks.block(query.block) else {
                    return false;
                };
                let literal = block
                    .instructions()
                    .iter()
                    .find(|instruction| Address::of(&instruction.output.local) == left)
                    .and_then(|instruction| match origin::instruction(&instruction.kind) {
                        Origin::Text(value) => Some(value),
                        _ => None,
                    });
                if let Some(literal) = literal {
                    if literal.starts_with(required.as_ref()) {
                        return true;
                    }
                    let Some(remainder) = required.strip_prefix(literal) else {
                        return false;
                    };
                    (right, Requirement::Prefix(remainder.to_owned().into()))
                } else {
                    (left, query.requirement.clone())
                }
            }
            _ => return false,
        };
        pending.push(Visit::Enter(Query {
            block: query.block,
            place: Place::local(local),
            requirement,
        }));
        true
    }

    fn same_place(&self, block: BlockId, address: Address, source: &Place) -> bool {
        Place::local(address)
            .normalize(block, self.blocks)
            .is_some_and(|place| place == *source)
    }
}

impl Requirement<'_> {
    fn is_empty(&self) -> bool {
        match self {
            Self::Length(length) => *length == 0,
            Self::Prefix(prefix) => prefix.is_empty(),
        }
    }

    fn covers(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Length(known), Self::Length(required)) => known >= required,
            (Self::Prefix(known), Self::Prefix(required)) => known.starts_with(required.as_ref()),
            _ => false,
        }
    }

    fn accepts_text(&self, value: &str) -> bool {
        matches!(self, Self::Prefix(prefix) if value.starts_with(prefix.as_ref()))
    }

    fn description(&self) -> String {
        match self {
            Self::Length(length) => format!("at least {length} list elements"),
            Self::Prefix(prefix) => format!("string prefix {prefix:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlockId, Blocks, BoolInstruction, BoolLocalId, GuardError, Guards, ParamLocal, Place,
        ProfiledInstruction, ProfiledInstructionKind, Query, Requirement, access,
    };
    use crate::plan::Text;
    use crate::plan::execution::graph::{
        BlockGraphExitId, BoolBranch, Edge, IntInstruction, IntListLocalId, IntLocalId, Jump,
        ListInstruction, ListLocal, ParamSlot, ProfiledBlock, ProfiledBlockGraph,
        StringInstruction, StringLocalId, Terminator, TypedListInstruction,
    };
    use crate::plan::execution::type_::{IntListTypeId, ListTypeId, ValueShapeId};
    use std::convert::Infallible;

    #[test]
    fn checks_real_list_and_string_pattern_projections() {
        let source = r#"
fn head(xs) { case xs { [] -> 0 [x, ..] -> x } }
fn second(xs) { case xs { [_, x, ..] -> x _ -> 0 } }
fn prefix(s) { case s { "pre" <> tail -> tail _ -> "" } }
fn asserted(xs) { let assert [_, x, ..tail] as whole = xs #(x, tail, whole) }
pub fn main() { #(head([42]), second([0, 42]), prefix("prefix"), asserted([0, 42, 43])) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let mut guarded = 0;
        for function in plan.program.functions.value_returns.int_functions.iter() {
            guarded += check_graph(function.body().block_graph());
        }
        for function in plan.program.functions.value_returns.string_functions.iter() {
            guarded += check_graph(function.body().block_graph());
        }
        for function in plan.program.functions.value_returns.tuple_functions.iter() {
            guarded += check_graph(function.body().block_graph());
        }
        assert!(guarded >= 3, "expected lowered list and string access");
    }

    fn check_graph(graph: &ProfiledBlockGraph<Infallible>) -> usize {
        let blocks = Blocks::admit(graph).unwrap();
        let guards = Guards { blocks: &blocks };
        let mut count = 0;
        for (index, block) in blocks.iter().enumerate() {
            for instruction in block.instructions() {
                guards.check(BlockId(index), instruction).unwrap();
                count += usize::from(access::instruction(&instruction.kind).unwrap().is_some());
            }
        }
        count
    }

    #[test]
    fn list_guards_require_the_right_operand_threshold_and_branch() {
        for (predicate, negate, minimum, expected) in [
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(0),
                    length: 2,
                },
                false,
                2,
                true,
            ),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(0),
                    length: 2,
                },
                false,
                3,
                false,
            ),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(1),
                    length: 2,
                },
                false,
                1,
                false,
            ),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(0),
                    length: 2,
                },
                true,
                1,
                false,
            ),
            (
                BoolInstruction::ListLengthEquals {
                    value: list(0),
                    length: 0,
                },
                true,
                1,
                true,
            ),
            (
                BoolInstruction::ListLengthEquals {
                    value: list(0),
                    length: 0,
                },
                true,
                2,
                false,
            ),
            (BoolInstruction::Value(true), false, 1, false),
        ] {
            for bypass in [false, true] {
                let graph = branch(
                    ParamLocal::List(list(0)),
                    ParamLocal::List(list(1)),
                    predicate.clone(),
                    negate,
                    bypass,
                );
                let blocks = Blocks::admit(&graph).unwrap();
                let guards = Guards { blocks: &blocks };
                assert_eq!(
                    guards.proves(Query {
                        block: BlockId(1),
                        place: Place::local(IntListLocalId(0).into()),
                        requirement: Requirement::Length(minimum)
                    }),
                    expected && !bypass
                );
                assert!(!guards.proves(Query {
                    block: BlockId(0),
                    place: Place::local(IntListLocalId(0).into()),
                    requirement: Requirement::Length(1)
                }));
            }
        }
    }

    #[test]
    fn prefixes_preserve_utf8_boundaries_and_cannot_use_an_unrelated_check() {
        for (predicate, negate, prefix, expected) in [
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(0),
                    prefix: Text::Static("\u{e9}-"),
                },
                false,
                "\u{e9}",
                true,
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(0),
                    prefix: Text::Static("pre"),
                },
                false,
                "prefix",
                false,
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(1),
                    prefix: Text::Static("pre"),
                },
                false,
                "pre",
                false,
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(0),
                    prefix: Text::Static("pre"),
                },
                true,
                "pre",
                false,
            ),
        ] {
            let graph = branch(
                ParamLocal::String(StringLocalId(0)),
                ParamLocal::String(StringLocalId(1)),
                predicate,
                negate,
                false,
            );
            let blocks = Blocks::admit(&graph).unwrap();
            let guards = Guards { blocks: &blocks };
            assert_eq!(
                guards.proves(Query {
                    block: BlockId(1),
                    place: Place::local(StringLocalId(0).into()),
                    requirement: Requirement::Prefix(prefix.into())
                }),
                expected
            );
            let read = instruction(
                ParamLocal::String(StringLocalId(1)),
                ProfiledInstructionKind::String(StringInstruction::DropPrefix {
                    value: StringLocalId(0),
                    prefix: Text::Owned(prefix.into()),
                }),
            );
            assert_eq!(guards.check(BlockId(1), &read).is_ok(), expected);
        }
    }

    fn branch(
        source: ParamLocal,
        other: ParamLocal,
        predicate: BoolInstruction,
        negate: bool,
        bypass: bool,
    ) -> ProfiledBlockGraph<Infallible> {
        let mut instructions = vec![instruction(
            ParamLocal::Bool(BoolLocalId(0)),
            ProfiledInstructionKind::Bool(predicate),
        )];
        if negate {
            instructions.push(instruction(
                ParamLocal::Bool(BoolLocalId(1)),
                ProfiledInstructionKind::Bool(BoolInstruction::Not(BoolLocalId(0))),
            ));
        }
        let edge = |target| Edge {
            target: BlockId(target),
            args: vec![source.clone()].into(),
        };
        ProfiledBlockGraph::from_parts(
            BlockId(0),
            vec![
                ProfiledBlock::new(
                    vec![slot(source.clone()), slot(other)],
                    instructions,
                    Terminator::BoolBranch(BoolBranch {
                        subject: BoolLocalId(usize::from(negate)),
                        true_: edge(1),
                        false_: edge(2),
                    }),
                ),
                ProfiledBlock::new(vec![slot(source.clone())], vec![], exit()),
                ProfiledBlock::new(
                    vec![slot(source.clone())],
                    vec![],
                    if bypass {
                        Terminator::Jump(Jump { edge: edge(1) })
                    } else {
                        exit()
                    },
                ),
            ],
        )
    }

    #[test]
    fn loops_cannot_strengthen_their_own_length_precondition() {
        let type_id = IntListTypeId {
            list_type: ListTypeId(0),
        };
        for (drop, expected) in [(0, true), (1, false), (usize::MAX, false)] {
            let graph = ProfiledBlockGraph::<Infallible>::from_parts(
                BlockId(0),
                vec![
                    ProfiledBlock::new(
                        vec![],
                        vec![
                            instruction(
                                ParamLocal::Int(IntLocalId(0)),
                                ProfiledInstructionKind::Int(IntInstruction::Value(
                                    num_bigint::BigInt::from(42).into(),
                                )),
                            ),
                            instruction(
                                ParamLocal::List(list(0)),
                                ProfiledInstructionKind::List(ListInstruction::Int(
                                    type_id,
                                    TypedListInstruction::Value(vec![IntLocalId(0)].into()),
                                )),
                            ),
                        ],
                        Terminator::Jump(Jump {
                            edge: Edge {
                                target: BlockId(1),
                                args: vec![ParamLocal::List(list(0))].into(),
                            },
                        }),
                    ),
                    ProfiledBlock::new(
                        vec![slot(ParamLocal::List(list(0)))],
                        vec![instruction(
                            ParamLocal::List(list(1)),
                            ProfiledInstructionKind::List(ListInstruction::Int(
                                type_id,
                                TypedListInstruction::DropFirst {
                                    list: IntListLocalId(0),
                                    count: drop,
                                },
                            )),
                        )],
                        Terminator::Jump(Jump {
                            edge: Edge {
                                target: BlockId(1),
                                args: vec![ParamLocal::List(list(1))].into(),
                            },
                        }),
                    ),
                ],
            );
            let blocks = Blocks::admit(&graph).unwrap();
            let guards = Guards { blocks: &blocks };
            assert_eq!(
                guards.proves(Query {
                    block: BlockId(1),
                    place: Place::local(IntListLocalId(0).into()),
                    requirement: Requirement::Length(1)
                }),
                expected
            );
            let read = instruction(
                ParamLocal::Int(IntLocalId(0)),
                ProfiledInstructionKind::Int(IntInstruction::ListIndex {
                    list: IntListLocalId(0),
                    index: usize::MAX,
                }),
            );
            assert_eq!(
                guards.check(BlockId(1), &read),
                Err(GuardError::LengthOverflow)
            );
        }
    }

    #[test]
    fn requirement_strength_preserves_lengths_and_complete_utf8_prefixes() {
        let cases = [
            (Requirement::Length(0), Requirement::Length(0), true),
            (Requirement::Length(2), Requirement::Length(1), true),
            (Requirement::Length(1), Requirement::Length(2), false),
            (
                Requirement::Prefix("".into()),
                Requirement::Prefix("a".into()),
                false,
            ),
            (
                Requirement::Prefix("\u{e9}-".into()),
                Requirement::Prefix("\u{e9}".into()),
                true,
            ),
            (
                Requirement::Prefix("a".into()),
                Requirement::Length(1),
                false,
            ),
            (
                Requirement::Length(1),
                Requirement::Prefix("a".into()),
                false,
            ),
        ];
        for (known, required, expected) in cases {
            assert_eq!(known.covers(&required), expected);
        }
        assert!(Requirement::Length(0).is_empty());
        assert!(!Requirement::Length(1).is_empty());
        assert!(Requirement::Prefix("".into()).is_empty());
        assert!(!Requirement::Prefix("a".into()).is_empty());
        assert_eq!(
            Requirement::Length(2).description(),
            "at least 2 list elements"
        );
        assert_eq!(
            Requirement::Prefix("a\n".into()).description(),
            "string prefix \"a\\n\""
        );
    }

    #[test]
    fn match_bindings_preserve_proven_lengths_and_trace_unproven_values() {
        use crate::plan::execution::graph::{
            Match, MatchEdge, MatchEdgeArgument, MatchPattern, MatchPatternBinding,
            MatchPatternList, MatchPatternListTail,
        };

        for (pattern, binding, literal, expected) in [
            (
                MatchPattern::Alias {
                    pattern: Box::new(MatchPattern::List(MatchPatternList::new(
                        vec![MatchPattern::Discard],
                        Some(MatchPatternListTail::Ignore),
                    )))
                    .into(),
                    binding: MatchPatternBinding::new(0),
                },
                0,
                false,
                true,
            ),
            (
                MatchPattern::Bind(MatchPatternBinding::new(0)),
                0,
                true,
                true,
            ),
            (
                MatchPattern::Bind(MatchPatternBinding::new(0)),
                0,
                false,
                false,
            ),
            (MatchPattern::Discard, 0, false, false),
        ] {
            let (params, instructions) = if literal {
                (
                    Vec::new(),
                    vec![
                        instruction(
                            ParamLocal::Int(IntLocalId(0)),
                            ProfiledInstructionKind::Int(IntInstruction::Value(
                                num_bigint::BigInt::from(42).into(),
                            )),
                        ),
                        instruction(
                            ParamLocal::List(list(0)),
                            ProfiledInstructionKind::List(ListInstruction::Int(
                                IntListTypeId {
                                    list_type: ListTypeId(0),
                                },
                                TypedListInstruction::Value(vec![IntLocalId(0)].into()),
                            )),
                        ),
                    ],
                )
            } else {
                (vec![slot(ParamLocal::List(list(0)))], Vec::new())
            };
            let graph = ProfiledBlockGraph::from_parts(
                BlockId(0),
                vec![
                    ProfiledBlock::new(
                        params,
                        instructions,
                        Terminator::Match(Match {
                            subject: ParamLocal::List(list(0)),
                            pattern,
                            success: MatchEdge::new(
                                BlockId(1),
                                vec![MatchEdgeArgument::Binding(binding)],
                            ),
                            failure: Edge::new(BlockId(2), Vec::new()),
                        }),
                    ),
                    ProfiledBlock::new(vec![slot(ParamLocal::List(list(0)))], Vec::new(), exit()),
                    ProfiledBlock::new(Vec::new(), Vec::new(), exit()),
                ],
            );
            let blocks = Blocks::admit(&graph).unwrap();
            assert_eq!(
                Guards { blocks: &blocks }.proves(Query {
                    block: BlockId(1),
                    place: Place::local(IntListLocalId(0).into()),
                    requirement: Requirement::Length(1),
                }),
                expected
            );
        }
    }

    #[test]
    fn raw_predicates_need_a_real_boolean_definition_and_matching_operand() {
        use super::Condition;

        let graph = ProfiledBlockGraph::from_parts(
            BlockId(0),
            vec![ProfiledBlock::new(
                vec![slot(ParamLocal::List(list(0)))],
                vec![
                    instruction(
                        ParamLocal::Bool(BoolLocalId(0)),
                        ProfiledInstructionKind::Int(IntInstruction::Value(
                            num_bigint::BigInt::from(42).into(),
                        )),
                    ),
                    instruction(
                        ParamLocal::Bool(BoolLocalId(1)),
                        ProfiledInstructionKind::Bool(BoolInstruction::Not(BoolLocalId(1))),
                    ),
                    instruction(
                        ParamLocal::Bool(BoolLocalId(2)),
                        ProfiledInstructionKind::Bool(BoolInstruction::Not(BoolLocalId(99))),
                    ),
                    instruction(
                        ParamLocal::Bool(BoolLocalId(3)),
                        ProfiledInstructionKind::Bool(BoolInstruction::ListLengthEquals {
                            value: list(0),
                            length: 1,
                        }),
                    ),
                ],
                exit(),
            )],
        );
        let blocks = Blocks::admit(&graph).unwrap();
        let guards = Guards { blocks: &blocks };
        let source = Place::local(IntListLocalId(0).into());
        for (block, index) in [(99, 0), (0, 0), (0, 1), (0, 2), (0, 99)] {
            assert!(!guards.boolean(
                BlockId(block),
                BoolLocalId(index),
                true,
                &source,
                &Requirement::Length(1),
            ));
            assert!(!guards.contradicts(
                BlockId(block),
                Condition::Bool {
                    subject: BoolLocalId(index),
                    truth: true
                },
            ));
        }
        assert!(!guards.boolean(
            BlockId(0),
            BoolLocalId(3),
            true,
            &source,
            &Requirement::Prefix("pre".into()),
        ));
        for (block, local) in [(99, 0), (0, 99)] {
            assert!(!guards.proves(Query {
                block: BlockId(block),
                place: Place::local(IntListLocalId(local).into()),
                requirement: Requirement::Length(1),
            }));
        }
    }

    #[test]
    fn string_and_nested_match_conditions_only_prove_the_selected_place() {
        use super::{Condition, Projection};
        use crate::plan::execution::graph::{
            Match, MatchEdge, MatchPattern, MatchPatternList, MatchPatternListTail, TupleLocalId,
        };

        let graph = ProfiledBlockGraph::<Infallible>::from_parts(
            BlockId(0),
            vec![ProfiledBlock::new(
                vec![
                    slot(ParamLocal::String(StringLocalId(0))),
                    slot(ParamLocal::Tuple {
                        local: TupleLocalId(0),
                        type_: vec![crate::plan::execution::type_::ValueType::List(ListTypeId(
                            0,
                        ))]
                        .into(),
                    }),
                ],
                Vec::new(),
                exit(),
            )],
        );
        let blocks = Blocks::admit(&graph).unwrap();
        let guards = Guards { blocks: &blocks };
        let prefix = Requirement::Prefix("pre".into());
        for (local, text, expected) in [
            (0, "prefix", true),
            (0, "other", false),
            (1, "prefix", false),
        ] {
            assert_eq!(
                guards.condition(
                    BlockId(0),
                    Condition::String {
                        subject: StringLocalId(0),
                        value: text
                    },
                    &Place::local(StringLocalId(local).into()),
                    &prefix,
                ),
                expected
            );
        }
        let matcher = Match {
            subject: ParamLocal::Tuple {
                local: TupleLocalId(0),
                type_: vec![crate::plan::execution::type_::ValueType::List(ListTypeId(
                    0,
                ))]
                .into(),
            },
            pattern: MatchPattern::Tuple(
                vec![MatchPattern::List(MatchPatternList::new(
                    vec![MatchPattern::Discard],
                    Some(MatchPatternListTail::Ignore),
                ))]
                .into(),
            ),
            success: MatchEdge::new(BlockId(0), Vec::new()),
            failure: Edge::new(BlockId(0), Vec::new()),
        };
        for (block, root, path, success, expected) in [
            (0, 0, vec![Projection::Tuple(0)], true, true),
            (0, 0, vec![Projection::Tuple(0)], false, false),
            (0, 0, vec![Projection::Tuple(1)], true, false),
            (0, 1, vec![Projection::Tuple(0)], true, false),
            (99, 0, vec![Projection::Tuple(0)], true, false),
        ] {
            assert_eq!(
                guards.condition(
                    BlockId(block),
                    Condition::Match {
                        matcher: &matcher,
                        success
                    },
                    &Place {
                        root: TupleLocalId(root).into(),
                        path
                    },
                    &Requirement::Length(1),
                ),
                expected
            );
        }
    }

    #[test]
    fn raw_edges_and_projection_cycles_cannot_supply_length_evidence() {
        use crate::plan::execution::graph::{TupleInstruction, TupleLocalId};

        for (cyclic, arguments) in [
            (false, Vec::new()),
            (
                true,
                vec![ParamLocal::Tuple {
                    local: TupleLocalId(0),
                    type_: vec![crate::plan::execution::type_::ValueType::List(ListTypeId(
                        0,
                    ))]
                    .into(),
                }],
            ),
        ] {
            let graph = ProfiledBlockGraph::from_parts(
                BlockId(0),
                vec![
                    ProfiledBlock::new(
                        Vec::new(),
                        if cyclic {
                            vec![instruction(
                                ParamLocal::Tuple {
                                    local: TupleLocalId(0),
                                    type_: vec![crate::plan::execution::type_::ValueType::List(
                                        ListTypeId(0),
                                    )]
                                    .into(),
                                },
                                ProfiledInstructionKind::Tuple(TupleInstruction::TupleIndex {
                                    tuple: TupleLocalId(0),
                                    index: 0,
                                }),
                            )]
                        } else {
                            Vec::new()
                        },
                        Terminator::Jump(Jump {
                            edge: Edge::new(BlockId(1), arguments),
                        }),
                    ),
                    ProfiledBlock::new(
                        vec![slot(ParamLocal::Tuple {
                            local: TupleLocalId(0),
                            type_: vec![crate::plan::execution::type_::ValueType::List(
                                ListTypeId(0),
                            )]
                            .into(),
                        })],
                        Vec::new(),
                        exit(),
                    ),
                ],
            );
            let blocks = Blocks::admit(&graph).unwrap();
            let guards = Guards { blocks: &blocks };
            assert!(!guards.proves(Query {
                block: BlockId(1),
                place: Place::local(TupleLocalId(0).into()),
                requirement: Requirement::Length(1),
            }));
            assert!(!guards.proves(Query {
                block: BlockId(0),
                place: Place::local(TupleLocalId(0).into()),
                requirement: Requirement::Length(1),
            }));
        }
    }

    #[test]
    fn contradiction_requires_evidence_for_the_predicate_and_selected_truth() {
        use super::Condition;

        let cases = [
            (BoolInstruction::Value(true), false, true),
            (BoolInstruction::Value(true), true, false),
            (BoolInstruction::Not(BoolLocalId(0)), false, false),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(0),
                    length: 2,
                },
                false,
                true,
            ),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(0),
                    length: 3,
                },
                false,
                false,
            ),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: list(0),
                    length: 2,
                },
                true,
                false,
            ),
            (
                BoolInstruction::ListLengthEquals {
                    value: list(0),
                    length: 1,
                },
                true,
                true,
            ),
            (
                BoolInstruction::ListLengthEquals {
                    value: list(0),
                    length: 2,
                },
                true,
                false,
            ),
            (
                BoolInstruction::ListLengthEquals {
                    value: list(0),
                    length: usize::MAX,
                },
                true,
                false,
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(0),
                    prefix: Text::Static("pre"),
                },
                false,
                true,
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(0),
                    prefix: Text::Static("other"),
                },
                false,
                false,
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(0),
                    prefix: Text::Static("pre"),
                },
                true,
                false,
            ),
        ];
        for (predicate, truth, expected) in cases {
            let graph = ProfiledBlockGraph::from_parts(
                BlockId(0),
                vec![ProfiledBlock::new(
                    Vec::new(),
                    vec![
                        instruction(
                            ParamLocal::Int(IntLocalId(0)),
                            ProfiledInstructionKind::Int(IntInstruction::Value(
                                num_bigint::BigInt::from(42).into(),
                            )),
                        ),
                        instruction(
                            ParamLocal::List(list(0)),
                            ProfiledInstructionKind::List(ListInstruction::Int(
                                IntListTypeId::new(ListTypeId(0)),
                                TypedListInstruction::Value(
                                    vec![IntLocalId(0), IntLocalId(0)].into(),
                                ),
                            )),
                        ),
                        instruction(
                            ParamLocal::String(StringLocalId(0)),
                            ProfiledInstructionKind::String(StringInstruction::Value(
                                "prefix".into(),
                            )),
                        ),
                        instruction(
                            ParamLocal::Bool(BoolLocalId(0)),
                            ProfiledInstructionKind::Bool(predicate),
                        ),
                        instruction(
                            ParamLocal::Bool(BoolLocalId(1)),
                            ProfiledInstructionKind::Bool(BoolInstruction::Not(BoolLocalId(0))),
                        ),
                    ],
                    exit(),
                )],
            );
            let blocks = Blocks::admit(&graph).unwrap();
            let guards = Guards { blocks: &blocks };
            assert_eq!(
                guards.contradicts(
                    BlockId(0),
                    Condition::Bool {
                        subject: BoolLocalId(0),
                        truth,
                    }
                ),
                expected
            );
            assert_eq!(
                guards.contradicts(
                    BlockId(0),
                    Condition::Bool {
                        subject: BoolLocalId(1),
                        truth: !truth,
                    }
                ),
                expected
            );
            assert!(!guards.contradicts(BlockId(0), Condition::Always));
            assert!(!guards.contradicts(
                BlockId(99),
                Condition::Bool {
                    subject: BoolLocalId(0),
                    truth,
                }
            ));
            assert!(!guards.contradicts(
                BlockId(0),
                Condition::Bool {
                    subject: BoolLocalId(99),
                    truth,
                }
            ));
        }
    }

    #[test]
    fn origins_either_discharge_a_requirement_or_preserve_its_exact_remaining_obligation() {
        use super::{Origin, Visit};
        let graph = ProfiledBlockGraph::from_parts(
            BlockId(0),
            vec![ProfiledBlock::new(
                Vec::new(),
                vec![
                    instruction(
                        ParamLocal::String(StringLocalId(0)),
                        ProfiledInstructionKind::String(StringInstruction::Value("pre".into())),
                    ),
                    instruction(
                        ParamLocal::String(StringLocalId(1)),
                        ProfiledInstructionKind::String(StringInstruction::Value("fix".into())),
                    ),
                    instruction(
                        ParamLocal::String(StringLocalId(2)),
                        ProfiledInstructionKind::String(StringInstruction::Concatenate {
                            left: StringLocalId(0),
                            right: StringLocalId(1),
                        }),
                    ),
                    instruction(
                        ParamLocal::String(StringLocalId(3)),
                        ProfiledInstructionKind::String(StringInstruction::DropPrefix {
                            value: StringLocalId(2),
                            prefix: "pre".into(),
                        }),
                    ),
                ],
                exit(),
            )],
        );
        let blocks = Blocks::admit(&graph).unwrap();
        let guards = Guards { blocks: &blocks };
        let first = StringLocalId(0).into();
        let second = StringLocalId(1).into();
        let joined = StringLocalId(2).into();
        let cases = [
            (
                Origin::List {
                    head: 2,
                    tail: None,
                },
                Requirement::Length(2),
                true,
                None,
            ),
            (
                Origin::List {
                    head: 2,
                    tail: None,
                },
                Requirement::Length(3),
                false,
                None,
            ),
            (
                Origin::List {
                    head: 2,
                    tail: Some(IntListLocalId(0).into()),
                },
                Requirement::Length(3),
                true,
                Some((IntListLocalId(0).into(), Requirement::Length(1))),
            ),
            (
                Origin::ListDrop {
                    source: IntListLocalId(0).into(),
                    count: 2,
                },
                Requirement::Length(3),
                true,
                Some((IntListLocalId(0).into(), Requirement::Length(5))),
            ),
            (
                Origin::ListDrop {
                    source: IntListLocalId(0).into(),
                    count: usize::MAX,
                },
                Requirement::Length(1),
                false,
                None,
            ),
            (
                Origin::Text("prefix"),
                Requirement::Prefix("pre".into()),
                true,
                None,
            ),
            (
                Origin::Text("other"),
                Requirement::Prefix("pre".into()),
                false,
                None,
            ),
            (
                Origin::TextDrop {
                    source: first,
                    prefix: "pre",
                },
                Requirement::Prefix("fix".into()),
                true,
                Some((first, Requirement::Prefix("prefix".into()))),
            ),
            (
                Origin::Concatenate {
                    left: first,
                    right: second,
                },
                Requirement::Prefix("pr".into()),
                true,
                None,
            ),
            (
                Origin::Concatenate {
                    left: first,
                    right: second,
                },
                Requirement::Prefix("prefix".into()),
                true,
                Some((second, Requirement::Prefix("fix".into()))),
            ),
            (
                Origin::Concatenate {
                    left: first,
                    right: second,
                },
                Requirement::Prefix("other".into()),
                false,
                None,
            ),
            (
                Origin::Concatenate {
                    left: joined,
                    right: second,
                },
                Requirement::Prefix("pre".into()),
                true,
                Some((joined, Requirement::Prefix("pre".into()))),
            ),
            (Origin::Unknown, Requirement::Length(1), false, None),
        ];
        for (origin, requirement, expected, obligation) in cases {
            let mut pending = Vec::new();
            let query = Query {
                block: BlockId(0),
                place: Place::local(first),
                requirement,
            };
            assert_eq!(guards.origin(query, origin, &mut pending), expected);
            let expected = obligation
                .into_iter()
                .map(|(root, requirement)| {
                    Visit::Enter(Query {
                        block: BlockId(0),
                        place: Place::local(root),
                        requirement,
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(pending, expected);
        }
        for (index, prefix, expected) in [
            (2, "prefix", true),
            (2, "other", false),
            (3, "fix", true),
            (3, "other", false),
            (3, "", true),
        ] {
            assert_eq!(
                guards.proves(Query {
                    block: BlockId(0),
                    place: Place::local(StringLocalId(index).into()),
                    requirement: Requirement::Prefix(prefix.into()),
                }),
                expected
            );
        }
        let mut pending = Vec::new();
        assert!(!guards.origin(
            Query {
                block: BlockId(99),
                place: Place::local(first),
                requirement: Requirement::Prefix("pre".into()),
            },
            Origin::Concatenate {
                left: first,
                right: second
            },
            &mut pending
        ));
        assert!(pending.is_empty());
    }

    fn list(index: usize) -> ListLocal {
        ListLocal::Int {
            local: IntListLocalId(index),
            type_id: IntListTypeId {
                list_type: ListTypeId(0),
            },
        }
    }

    fn slot(local: ParamLocal) -> ParamSlot {
        ParamSlot {
            local,
            shape: ValueShapeId(0),
        }
    }

    fn instruction(
        local: ParamLocal,
        kind: ProfiledInstructionKind<Infallible>,
    ) -> ProfiledInstruction<Infallible> {
        ProfiledInstruction {
            output: slot(local),
            kind,
        }
    }

    fn exit() -> Terminator {
        Terminator::Exit(BlockGraphExitId(0))
    }
}
