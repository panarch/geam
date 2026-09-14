mod projection;
pub(super) use projection::instruction as projected_input;

use super::block::Blocks;
use super::local::Address;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{BlockId, BlockView, MatchPattern, MatchPatternListTail};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Place {
    pub(super) root: Address,
    pub(super) path: Vec<Projection>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Projection {
    Tuple(usize),
    Custom(usize),
    List(usize),
}

impl Place {
    pub(super) fn local(root: Address) -> Self {
        Self {
            root,
            path: Vec::new(),
        }
    }

    pub(super) fn with_root(&self, root: Address) -> Self {
        Self {
            root,
            path: self.path.clone(),
        }
    }

    // Projected reads in different blocks can still name the same immutable
    // field. Keep that path while tracing a block argument to its predecessor.
    pub(super) fn normalize<Graph: ExecutionGraphProfile>(
        self,
        block: BlockId,
        blocks: &Blocks<'_, Graph>,
    ) -> Option<Self> {
        self.in_block(blocks.find_block(block)?)
    }

    pub(super) fn in_block<Graph: ExecutionGraphProfile>(
        mut self,
        block: BlockView<'_, Graph>,
    ) -> Option<Self> {
        let mut visited = HashSet::new();
        loop {
            if !visited.insert(self.root) {
                return None;
            }
            let Some(instruction) = block
                .instructions()
                .iter()
                .find(|value| Address::of(&value.output.local) == self.root)
            else {
                return Some(self);
            };
            let Some((root, field)) = projection::instruction(&instruction.kind) else {
                return Some(self);
            };
            self.root = root;
            self.path.insert(0, field);
        }
    }
}

pub(super) fn pattern_at<'data>(
    mut pattern: &'data MatchPattern,
    path: &[Projection],
) -> Option<&'data MatchPattern> {
    let mut visited = HashSet::new();
    for field in path {
        while let MatchPattern::Alias { pattern: inner, .. } = pattern {
            if !visited.insert(pattern as *const MatchPattern) {
                return None;
            }
            pattern = inner;
        }
        if !visited.insert(pattern as *const MatchPattern) {
            return None;
        }
        pattern = match (field, pattern) {
            (Projection::Tuple(index), MatchPattern::Tuple(fields))
            | (Projection::Custom(index), MatchPattern::Custom { fields, .. }) => {
                fields.get(*index)?
            }
            (Projection::List(index), MatchPattern::List(list)) => list.elements.get(*index)?,
            _ => return None,
        };
    }
    Some(pattern)
}

pub(super) fn binding_path(
    pattern: &MatchPattern,
    binding_index: usize,
    projection: &[Projection],
) -> Option<Vec<Projection>> {
    let mut pending = vec![(pattern, Vec::new())];
    let mut visited = HashSet::new();
    while let Some((pattern, mut path)) = pending.pop() {
        if !visited.insert(pattern as *const MatchPattern) {
            return None;
        }
        match pattern {
            MatchPattern::Bind(binding) if binding.index == binding_index => {
                path.extend_from_slice(projection);
                return Some(path);
            }
            MatchPattern::Alias { pattern, binding } => {
                if binding.index == binding_index {
                    path.extend_from_slice(projection);
                    return Some(path);
                }
                pending.push((pattern, path));
            }
            MatchPattern::Tuple(fields) | MatchPattern::Custom { fields, .. } => {
                for (index, field) in fields.iter().enumerate() {
                    let mut path = path.clone();
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
                    && binding.index == binding_index
                {
                    let (Projection::List(index), rest) = projection.split_first()? else {
                        return None;
                    };
                    path.push(Projection::List(index.checked_add(list.elements.len())?));
                    path.extend_from_slice(rest);
                    return Some(path);
                }
                for (index, field) in list.elements.iter().enumerate() {
                    let mut path = path.clone();
                    path.push(Projection::List(index));
                    pending.push((field, path));
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Blocks, MatchPattern, Place, Projection, binding_path, pattern_at};
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockHeader, BlockId, IntLocalId, MatchPatternBinding, MatchPatternList,
        MatchPatternListTail, ProfiledBlockGraph, ProfiledInstructionKind, Terminator,
        TupleInstruction, TupleLocalId,
    };
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};

    #[test]
    fn normalizing_projections_keeps_the_original_parameter_and_ordered_field_path() {
        let source = r#"
fn project(value: #(#(Int, Bool), String)) { value.0.0 }
pub fn main() { project(#(#(42, True), "text")) }
"#;
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let function = plan
            .program
            .functions
            .value_returns
            .int_functions
            .iter()
            .find(|function| function.entry().parameter_count == 1)
            .unwrap();
        let graph = function.body().block_graph();
        assert_eq!(graph.blocks.len(), 1);
        let mut raw = ProfiledBlockGraph {
            entry: graph.entry,
            blocks: vec![BlockHeader {
                params: 0..graph.params.len(),
                instructions: 0..graph.instructions.len(),
                terminator: Terminator::Exit(BlockGraphExitId(0)),
            }]
            .into(),
            params: graph.params.clone(),
            instructions: graph.instructions.clone(),
        };
        let blocks = Blocks::admit(&raw).unwrap();
        let root = TupleLocalId(0).into();
        let expected = Place {
            root,
            path: vec![Projection::Tuple(0), Projection::Tuple(0)],
        };
        assert_eq!(
            Place::local(IntLocalId(0).into()).normalize(BlockId(0), &blocks),
            Some(expected.clone())
        );
        assert_eq!(
            Place::local(root).normalize(BlockId(0), &blocks),
            Some(Place::local(root))
        );
        assert_eq!(
            expected.with_root(TupleLocalId(5).into()),
            Place {
                root: TupleLocalId(5).into(),
                path: vec![Projection::Tuple(0), Projection::Tuple(0)],
            }
        );
        assert_eq!(Place::local(root).normalize(BlockId(99), &blocks), None);
        let mut instructions = raw.instructions.to_vec();
        instructions[0].kind = ProfiledInstructionKind::Tuple(TupleInstruction::TupleIndex {
            tuple: TupleLocalId(1),
            index: 0,
        });
        raw.instructions = instructions.into();
        let blocks = Blocks::admit(&raw).unwrap();
        assert_eq!(
            Place::local(IntLocalId(0).into()).normalize(BlockId(0), &blocks),
            None
        );

        let main = plan
            .program
            .functions
            .value_returns
            .int_functions
            .iter()
            .find(|function| function.entry().parameter_count == 0)
            .unwrap();
        let blocks = Blocks::admit(main.body().block_graph()).unwrap();
        assert_eq!(
            Place::local(IntLocalId(0).into()).normalize(BlockId(0), &blocks),
            Some(Place::local(IntLocalId(0).into()))
        );
    }

    #[test]
    fn pattern_projection_follows_aliases_but_keeps_custom_tuple_and_list_steps_distinct() {
        static LEAF: MatchPattern = MatchPattern::Bool(true);
        let pattern = MatchPattern::Alias {
            binding: MatchPatternBinding::new(0),
            pattern: Box::new(MatchPattern::Tuple(
                vec![MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        type_id: CustomTypeId(0),
                        index: 0,
                    },
                    fields: vec![MatchPattern::List(MatchPatternList::new(
                        vec![MatchPattern::Alias {
                            pattern: Node::Static(&LEAF),
                            binding: MatchPatternBinding::new(1),
                        }],
                        None,
                    ))]
                    .into(),
                }]
                .into(),
            ))
            .into(),
        };
        assert!(std::ptr::eq(pattern_at(&pattern, &[]).unwrap(), &pattern));
        let path = [
            Projection::Tuple(0),
            Projection::Custom(0),
            Projection::List(0),
        ];
        let selected = pattern_at(&pattern, &path).unwrap();
        assert_eq!(binding_path(selected, 1, &[]), Some(Vec::new()));
        for path in [
            vec![Projection::Tuple(1)],
            vec![Projection::Custom(0)],
            vec![Projection::Tuple(0), Projection::Tuple(0)],
            vec![Projection::Tuple(0), Projection::Custom(1)],
            vec![
                Projection::Tuple(0),
                Projection::Custom(0),
                Projection::List(1),
            ],
            vec![
                Projection::Tuple(0),
                Projection::Custom(0),
                Projection::List(0),
                Projection::Tuple(0),
            ],
        ] {
            assert!(pattern_at(&pattern, &path).is_none());
        }
        static CYCLE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&CYCLE),
            binding: MatchPatternBinding { index: 0 },
        };
        assert!(pattern_at(&CYCLE, &[Projection::Tuple(0)]).is_none());
        static TUPLE_CYCLE: MatchPattern =
            MatchPattern::Tuple(Table::Static(&[MatchPattern::Alias {
                pattern: Node::Static(&TUPLE_CYCLE),
                binding: MatchPatternBinding { index: 0 },
            }]));
        assert!(pattern_at(&TUPLE_CYCLE, &[Projection::Tuple(0), Projection::Tuple(0)]).is_none());
    }

    #[test]
    fn bindings_preserve_alias_paths_and_translate_list_tail_offsets() {
        let pattern = MatchPattern::Tuple(
            vec![
                MatchPattern::Alias {
                    binding: MatchPatternBinding::new(0),
                    pattern: Box::new(MatchPattern::Bind(MatchPatternBinding::new(1))).into(),
                },
                MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        type_id: CustomTypeId(0),
                        index: 0,
                    },
                    fields: vec![MatchPattern::List(MatchPatternList::new(
                        vec![
                            MatchPattern::Discard,
                            MatchPattern::Bind(MatchPatternBinding::new(2)),
                        ],
                        Some(MatchPatternListTail::Bind(MatchPatternBinding::new(3))),
                    ))]
                    .into(),
                },
            ]
            .into(),
        );
        for binding in [0, 1] {
            assert_eq!(
                binding_path(&pattern, binding, &[Projection::List(4)]),
                Some(vec![Projection::Tuple(0), Projection::List(4)])
            );
        }
        assert_eq!(
            binding_path(&pattern, 2, &[]),
            Some(vec![
                Projection::Tuple(1),
                Projection::Custom(0),
                Projection::List(1)
            ])
        );
        assert_eq!(
            binding_path(&pattern, 3, &[Projection::List(4), Projection::Tuple(0)]),
            Some(vec![
                Projection::Tuple(1),
                Projection::Custom(0),
                Projection::List(6),
                Projection::Tuple(0)
            ])
        );
        for projection in [
            vec![],
            vec![Projection::Tuple(0)],
            vec![Projection::List(usize::MAX)],
        ] {
            assert_eq!(binding_path(&pattern, 3, &projection), None);
        }
        assert_eq!(binding_path(&pattern, 4, &[]), None);
        let discarded_tail = MatchPattern::List(MatchPatternList::new(
            vec![MatchPattern::Bind(MatchPatternBinding::new(0))],
            Some(MatchPatternListTail::Ignore),
        ));
        assert_eq!(
            binding_path(&discarded_tail, 0, &[]),
            Some(vec![Projection::List(0)])
        );
        assert_eq!(binding_path(&discarded_tail, 1, &[]), None);
        assert_eq!(
            binding_path(
                &MatchPattern::List(MatchPatternList::new(vec![], None)),
                0,
                &[]
            ),
            None
        );
        static CYCLE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&CYCLE),
            binding: MatchPatternBinding { index: 0 },
        };
        assert_eq!(binding_path(&CYCLE, 1, &[]), None);
    }
}
