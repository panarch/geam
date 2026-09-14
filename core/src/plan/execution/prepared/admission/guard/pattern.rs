use super::super::place::{Projection, pattern_at};
use super::Requirement;
use crate::plan::execution::graph::{MatchPattern, MatchPatternListTail};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BindingProof<'data> {
    Proven,
    Source {
        path: Vec<Projection>,
        requirement: Requirement<'data>,
    },
    Unknown,
}

pub(super) fn establishes(
    pattern: &MatchPattern,
    success: bool,
    requirement: &Requirement<'_>,
) -> bool {
    let mut pattern = pattern;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(pattern as *const MatchPattern) {
            return false;
        }
        match pattern {
            MatchPattern::Alias { pattern: inner, .. } => pattern = inner,
            MatchPattern::List(list) => {
                return match requirement {
                    Requirement::Length(length) if success => list.elements.len() >= *length,
                    Requirement::Length(length) => {
                        list.elements.is_empty() && list.tail.is_none() && *length <= 1
                    }
                    Requirement::Prefix(_) => false,
                };
            }
            MatchPattern::String(value) => {
                return success && requirement.accepts_text(value.as_str());
            }
            MatchPattern::StringPrefix { prefix, .. } => {
                return success && requirement.accepts_text(prefix.as_str());
            }
            _ => return false,
        }
    }
}

pub(super) fn binding<'data>(
    pattern: &'data MatchPattern,
    index: usize,
    path: &[Projection],
    requirement: &Requirement<'data>,
) -> BindingProof<'data> {
    let mut pending = vec![(pattern, Vec::new())];
    let mut visited = HashSet::new();
    while let Some((pattern, mut source)) = pending.pop() {
        if !visited.insert(pattern as *const MatchPattern) {
            return BindingProof::Unknown;
        }
        match pattern {
            MatchPattern::Bind(binding) if binding.index == index => {
                source.extend_from_slice(path);
                return BindingProof::Source {
                    path: source,
                    requirement: requirement.clone(),
                };
            }
            MatchPattern::Alias { pattern, binding } => {
                if binding.index == index {
                    if pattern_at(pattern, path)
                        .is_some_and(|pattern| establishes(pattern, true, requirement))
                    {
                        return BindingProof::Proven;
                    }
                    source.extend_from_slice(path);
                    return BindingProof::Source {
                        path: source,
                        requirement: requirement.clone(),
                    };
                }
                pending.push((pattern, source));
            }
            MatchPattern::Tuple(fields) | MatchPattern::Custom { fields, .. } => {
                for (index, field) in fields.iter().enumerate() {
                    let mut path = source.clone();
                    path.push(if matches!(pattern, MatchPattern::Tuple(_)) {
                        Projection::Tuple(index)
                    } else {
                        Projection::Custom(index)
                    });
                    pending.push((field, path));
                }
            }
            MatchPattern::List(list) => {
                if let Some(MatchPatternListTail::Bind(binding)) = &list.tail
                    && binding.index == index
                {
                    if let Some((Projection::List(index), rest)) = path.split_first() {
                        let Some(index) = index.checked_add(list.elements.len()) else {
                            return BindingProof::Unknown;
                        };
                        source.push(Projection::List(index));
                        source.extend_from_slice(rest);
                        return BindingProof::Source {
                            path: source,
                            requirement: requirement.clone(),
                        };
                    }
                    if path.is_empty()
                        && let Requirement::Length(length) = requirement
                    {
                        let Some(length) = length.checked_add(list.elements.len()) else {
                            return BindingProof::Unknown;
                        };
                        return BindingProof::Source {
                            path: source,
                            requirement: Requirement::Length(length),
                        };
                    }
                    return BindingProof::Unknown;
                }
                for (index, field) in list.elements.iter().enumerate() {
                    let mut path = source.clone();
                    path.push(Projection::List(index));
                    pending.push((field, path));
                }
            }
            MatchPattern::StringPrefix {
                prefix,
                left,
                right,
            } if path.is_empty() => {
                if left.as_ref().is_some_and(|binding| binding.index == index) {
                    return if requirement.accepts_text(prefix.as_str()) {
                        BindingProof::Proven
                    } else {
                        BindingProof::Unknown
                    };
                }
                if right.as_ref().is_some_and(|binding| binding.index == index)
                    && let Requirement::Prefix(required) = requirement
                {
                    return BindingProof::Source {
                        path: source,
                        requirement: Requirement::Prefix(
                            format!("{}{required}", prefix.as_str()).into(),
                        ),
                    };
                }
            }
            _ => {}
        }
    }
    BindingProof::Unknown
}

#[cfg(test)]
mod tests {
    use super::{BindingProof, Requirement, binding, establishes};
    use crate::plan::execution::graph::{
        MatchPattern, MatchPatternBinding, MatchPatternList, MatchPatternListTail,
    };
    use crate::plan::execution::prepared::admission::place::Projection;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};

    #[test]
    fn successful_and_failed_list_patterns_prove_only_their_length_constraints() {
        for (elements, tail, success, minimum, expected) in [
            (0, None, false, 1, true),
            (0, None, false, 2, false),
            (0, None, true, 1, false),
            (1, None, true, 1, true),
            (1, None, false, 1, false),
            (1, Some(MatchPatternListTail::Ignore), true, 1, true),
            (0, Some(MatchPatternListTail::Ignore), false, 1, false),
        ] {
            let pattern = MatchPattern::List(MatchPatternList::new(
                vec![MatchPattern::Discard; elements],
                tail,
            ));
            assert_eq!(
                establishes(&pattern, success, &Requirement::Length(minimum)),
                expected
            );
            assert!(!establishes(
                &pattern,
                success,
                &Requirement::Prefix("pre".into())
            ));
        }
    }

    #[test]
    fn text_patterns_prove_prefixes_only_on_the_success_path() {
        for pattern in [
            MatchPattern::String("prefix".into()),
            MatchPattern::StringPrefix {
                prefix: "prefix".into(),
                left: None,
                right: None,
            },
            MatchPattern::Alias {
                pattern: Box::new(MatchPattern::String("prefix".into())).into(),
                binding: MatchPatternBinding::new(0),
            },
        ] {
            assert!(establishes(
                &pattern,
                true,
                &Requirement::Prefix("pre".into())
            ));
            assert!(!establishes(
                &pattern,
                false,
                &Requirement::Prefix("pre".into())
            ));
            assert!(!establishes(
                &pattern,
                true,
                &Requirement::Prefix("other".into())
            ));
            assert!(!establishes(&pattern, true, &Requirement::Length(1)));
        }
        assert!(!establishes(
            &MatchPattern::Discard,
            true,
            &Requirement::Length(1)
        ));
    }

    #[test]
    fn bindings_preserve_nested_tuple_custom_and_list_projection_paths() {
        let requirement = Requirement::Length(2);
        let direct = MatchPattern::Bind(MatchPatternBinding::new(4));
        assert_eq!(
            binding(&direct, 4, &[Projection::Tuple(1)], &requirement),
            BindingProof::Source {
                path: vec![Projection::Tuple(1)],
                requirement: Requirement::Length(2)
            }
        );
        assert_eq!(
            binding(&direct, 3, &[], &requirement),
            BindingProof::Unknown
        );

        let nested = MatchPattern::Tuple(
            vec![MatchPattern::Custom {
                constructor: CustomConstructorId {
                    type_id: CustomTypeId(0),
                    index: 0,
                },
                fields: vec![
                    MatchPattern::Discard,
                    MatchPattern::List(MatchPatternList::new(
                        vec![
                            MatchPattern::Bind(MatchPatternBinding::new(4)),
                            MatchPattern::Discard,
                        ],
                        None,
                    )),
                ]
                .into(),
            }]
            .into(),
        );
        assert_eq!(
            binding(&nested, 4, &[Projection::Tuple(1)], &requirement),
            BindingProof::Source {
                path: vec![
                    Projection::Tuple(0),
                    Projection::Custom(1),
                    Projection::List(0),
                    Projection::Tuple(1)
                ],
                requirement: Requirement::Length(2),
            }
        );
        assert_eq!(
            binding(&nested, 99, &[], &requirement),
            BindingProof::Unknown
        );
    }

    #[test]
    fn an_alias_uses_pattern_evidence_or_forwards_the_original_requirement() {
        let pattern = MatchPattern::Alias {
            pattern: Box::new(MatchPattern::List(MatchPatternList::new(
                vec![MatchPattern::Bind(MatchPatternBinding::new(1))],
                None,
            )))
            .into(),
            binding: MatchPatternBinding::new(0),
        };
        assert_eq!(
            binding(&pattern, 0, &[], &Requirement::Length(1)),
            BindingProof::Proven
        );
        assert_eq!(
            binding(&pattern, 0, &[], &Requirement::Length(2)),
            BindingProof::Source {
                path: vec![],
                requirement: Requirement::Length(2)
            }
        );
        assert_eq!(
            binding(&pattern, 0, &[Projection::List(9)], &Requirement::Length(1)),
            BindingProof::Source {
                path: vec![Projection::List(9)],
                requirement: Requirement::Length(1)
            }
        );
        assert_eq!(
            binding(&pattern, 1, &[], &Requirement::Length(1)),
            BindingProof::Source {
                path: vec![Projection::List(0)],
                requirement: Requirement::Length(1)
            }
        );
    }

    #[test]
    fn list_tail_bindings_translate_offsets_without_overflow_or_unrelated_proofs() {
        let pattern = MatchPattern::List(MatchPatternList::new(
            vec![
                MatchPattern::Bind(MatchPatternBinding::new(0)),
                MatchPattern::Discard,
            ],
            Some(MatchPatternListTail::Bind(MatchPatternBinding::new(1))),
        ));
        assert_eq!(
            binding(&pattern, 1, &[], &Requirement::Length(3)),
            BindingProof::Source {
                path: vec![],
                requirement: Requirement::Length(5)
            }
        );
        assert_eq!(
            binding(
                &pattern,
                1,
                &[Projection::List(1), Projection::Tuple(0)],
                &Requirement::Length(3)
            ),
            BindingProof::Source {
                path: vec![Projection::List(3), Projection::Tuple(0)],
                requirement: Requirement::Length(3)
            }
        );
        assert_eq!(
            binding(
                &pattern,
                1,
                &[Projection::Custom(0)],
                &Requirement::Length(1)
            ),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(&pattern, 1, &[], &Requirement::Prefix("x".into())),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(&pattern, 1, &[], &Requirement::Length(usize::MAX)),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(
                &pattern,
                1,
                &[Projection::List(usize::MAX)],
                &Requirement::Length(1)
            ),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(&pattern, 0, &[], &Requirement::Length(1)),
            BindingProof::Source {
                path: vec![Projection::List(0)],
                requirement: Requirement::Length(1)
            }
        );
        let ignored = MatchPattern::List(MatchPatternList::new(
            vec![],
            Some(MatchPatternListTail::Ignore),
        ));
        assert_eq!(
            binding(&ignored, 1, &[], &Requirement::Length(1)),
            BindingProof::Unknown
        );
    }

    #[test]
    fn string_prefix_bindings_distinguish_the_literal_prefix_and_remaining_suffix() {
        let pattern = MatchPattern::StringPrefix {
            prefix: "pre".into(),
            left: Some(MatchPatternBinding::new(0)),
            right: Some(MatchPatternBinding::new(1)),
        };
        assert_eq!(
            binding(&pattern, 0, &[], &Requirement::Prefix("pr".into())),
            BindingProof::Proven
        );
        assert_eq!(
            binding(&pattern, 0, &[], &Requirement::Prefix("other".into())),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(&pattern, 1, &[], &Requirement::Prefix("fix".into())),
            BindingProof::Source {
                path: vec![],
                requirement: Requirement::Prefix("prefix".into())
            }
        );
        assert_eq!(
            binding(&pattern, 1, &[], &Requirement::Length(1)),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(
                &pattern,
                1,
                &[Projection::Tuple(0)],
                &Requirement::Prefix("fix".into())
            ),
            BindingProof::Unknown
        );
        assert_eq!(
            binding(&pattern, 2, &[], &Requirement::Prefix("fix".into())),
            BindingProof::Unknown
        );
        let unbound = MatchPattern::StringPrefix {
            prefix: "pre".into(),
            left: None,
            right: None,
        };
        assert_eq!(
            binding(&unbound, 0, &[], &Requirement::Prefix("fix".into())),
            BindingProof::Unknown
        );
    }

    #[test]
    fn cyclic_aliases_and_repeated_unrelated_nodes_cannot_supply_a_proof() {
        static CYCLE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&CYCLE),
            binding: MatchPatternBinding { index: 0 },
        };
        assert!(!establishes(&CYCLE, true, &Requirement::Length(1)));
        assert_eq!(
            binding(&CYCLE, 1, &[], &Requirement::Length(1)),
            BindingProof::Unknown
        );
        static UNBOUND: MatchPattern = MatchPattern::Discard;
        let shared = MatchPattern::Tuple(Table::Owned(Box::new([
            MatchPattern::Alias {
                pattern: Node::Static(&UNBOUND),
                binding: MatchPatternBinding::new(0),
            },
            MatchPattern::Alias {
                pattern: Node::Static(&UNBOUND),
                binding: MatchPatternBinding::new(1),
            },
        ])));
        assert_eq!(
            binding(&shared, 2, &[], &Requirement::Length(1)),
            BindingProof::Unknown
        );
    }
}
