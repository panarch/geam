use super::block::Blocks;
use super::guard::Guards;
use super::incoming::{self, Condition, Input};
use super::local::{Address, Locals};
use super::place::{self, Place, Projection};
use super::type_::Types;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{BlockId, MatchPattern, ParamLocal, ParamSlot};
use crate::plan::execution::type_::custom::FieldRefinement;
use crate::plan::execution::type_::{
    CustomConstructorRefinement, ValueShapeDescriptor, ValueShapeId,
};
use std::collections::{HashMap, HashSet};

pub(super) struct Control<'graph, 'data, Graph: ExecutionGraphProfile> {
    pub(super) blocks: &'graph Blocks<'data, Graph>,
    pub(super) types: &'graph Types<'data>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ConstructorFact {
    Is(usize),
    IsNot(usize),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Query {
    block: BlockId,
    place: Place,
    fact: ConstructorFact,
    assumptions: Vec<Assumption>,
}

// Constructor hypotheses are relative to the query's immutable root. They are
// used only while proving that a failed nested pattern excludes its parent.
#[derive(Clone, PartialEq, Eq, Hash)]
struct Assumption {
    path: Vec<Projection>,
    constructor: usize,
}

enum Visit {
    Enter(Query),
    Leave(Query),
}

enum PatternVisit<'data> {
    Enter(&'data MatchPattern, Place, Vec<Assumption>),
    Leave(*const MatchPattern),
}

impl<'data, Graph: ExecutionGraphProfile> Control<'_, 'data, Graph> {
    pub(super) fn refine_projection(
        &self,
        block: BlockId,
        slot: &'data ParamSlot,
        locals: &mut Locals<'data>,
    ) {
        let original = Place::local(Address::of(&slot.local));
        if original
            .clone()
            .normalize(block, self.blocks)
            .is_some_and(|place| place != original)
        {
            self.refine(block, slot, locals);
        }
    }

    pub(super) fn refine(
        &self,
        block: BlockId,
        slot: &'data ParamSlot,
        locals: &mut Locals<'data>,
    ) {
        let Ok(ValueShapeDescriptor::Custom(id)) = self.types.shape(slot.shape) else {
            return;
        };
        let shape = &self.types.shapes.custom_shapes[id.0];
        let type_ = &self.types.customs.types[shape.type_id.index()];
        let mut possible = Vec::new();
        for constructor in type_.constructors.iter() {
            let index = constructor.id.index;
            if self.proves(block, &slot.local, ConstructorFact::Is(index)) {
                locals.set_constructor(&slot.local, index);
                locals.restrict_constructors(&slot.local, vec![index]);
                return;
            }
            if !self.proves(block, &slot.local, ConstructorFact::IsNot(index)) {
                possible.push(index);
            }
        }
        if possible.len() == 1 && type_.constructors.len() == type_.constructor_count {
            locals.set_constructor(&slot.local, possible[0]);
        }
        if !possible.is_empty() && possible.len() != type_.constructors.len() {
            locals.restrict_constructors(&slot.local, possible);
        }
    }

    // Every incoming path must establish the fact. Constructor exclusions prove
    // an exact variant only against the original declaration, not a sparse subset.
    fn proves(&self, block: BlockId, local: &'data ParamLocal, fact: ConstructorFact) -> bool {
        let mut pending = vec![Visit::Enter(Query {
            block,
            place: Place::local(Address::of(local)),
            fact,
            assumptions: Vec::new(),
        })];
        let mut active = HashMap::new();
        let mut complete = HashSet::new();
        while let Some(visit) = pending.pop() {
            let (query, block) = match visit {
                Visit::Enter(query) => {
                    let Some(block) = self.blocks.find_block(query.block) else {
                        return false;
                    };
                    let Some(query) = query.normalize(self.blocks) else {
                        return false;
                    };
                    (query, block)
                }
                Visit::Leave(query) => {
                    active.remove(&(query.block, query.place.root, query.fact));
                    complete.insert(query);
                    continue;
                }
            };
            if query.assumptions.iter().any(|assumption| {
                assumption.path == query.place.path && query.fact.holds(assumption.constructor)
            }) || complete.contains(&query)
            {
                continue;
            }
            let key = (query.block, query.place.root, query.fact);
            if let Some((path, assumptions)) = active.get(&key) {
                if path == &query.place.path && assumptions == &query.assumptions {
                    continue;
                }
                return false;
            }
            active.insert(key, (query.place.path.clone(), query.assumptions.clone()));
            pending.push(Visit::Leave(query.clone()));
            let parameter = block
                .params()
                .iter()
                .position(|slot| Address::of(&slot.local) == query.place.root);
            let slot = parameter.map(|index| &block.params()[index]).or_else(|| {
                block
                    .instructions()
                    .iter()
                    .map(|instruction| &instruction.output)
                    .find(|slot| Address::of(&slot.local) == query.place.root)
            });
            let Some(slot) = slot else { return false };
            if let Some(shape) =
                self.projected_shape(slot.shape, &query.place.path, &query.assumptions)
                && let Ok(ValueShapeDescriptor::Custom(id)) = self.types.shape(shape)
            {
                let shape = &self.types.shapes.custom_shapes[id.0];
                if let CustomConstructorRefinement::Exact(index) = shape.constructor
                    && query.fact.holds(index)
                {
                    continue;
                }
                let type_ = &self.types.customs.types[shape.type_id.index()];
                if let ConstructorFact::Is(index) = query.fact
                    && type_.constructors.len() == type_.constructor_count
                    && type_
                        .constructors
                        .iter()
                        .any(|constructor| constructor.id.index == index)
                {
                    pending.extend(
                        type_
                            .constructors
                            .iter()
                            .filter(|constructor| constructor.id.index != index)
                            .map(|constructor| {
                                Visit::Enter(Query {
                                    fact: ConstructorFact::IsNot(constructor.id.index),
                                    ..query.clone()
                                })
                            }),
                    );
                    continue;
                }
            }
            let Some(index) = parameter else { return false };
            if query.block == self.blocks.entry() {
                return false;
            }
            let Some(inputs) = incoming::parameter(self.blocks, query.block, index) else {
                return false;
            };
            for input in inputs {
                if (Guards {
                    blocks: self.blocks,
                })
                .contradicts(input.block, input.condition)
                {
                    continue;
                }
                let mut source = query.clone();
                source.block = input.block;
                match input.value {
                    Input::Local(local) => source.place.root = Address::of(local),
                    Input::Binding { index, matcher } => {
                        source.place.root = Address::of(&matcher.subject);
                        for path in std::iter::once(&mut source.place.path).chain(
                            source
                                .assumptions
                                .iter_mut()
                                .map(|assumption| &mut assumption.path),
                        ) {
                            let Some(mapped) = place::binding_path(&matcher.pattern, index, path)
                            else {
                                return false;
                            };
                            *path = mapped;
                        }
                    }
                }
                let Some(source) = source.normalize(self.blocks) else {
                    return false;
                };
                if let Some(requirements) = self.condition(
                    input.block,
                    input.condition,
                    &source.place,
                    source.fact,
                    &source.assumptions,
                ) {
                    pending.extend(requirements.into_iter().map(Visit::Enter));
                } else {
                    pending.push(Visit::Enter(source));
                }
            }
        }
        true
    }

    fn projected_shape(
        &self,
        mut shape: ValueShapeId,
        path: &[Projection],
        assumptions: &[Assumption],
    ) -> Option<ValueShapeId> {
        for (depth, projection) in path.iter().enumerate() {
            shape = match (projection, self.types.shapes.shapes.get(shape.index())?) {
                (Projection::Tuple(index), ValueShapeDescriptor::Tuple(fields)) => {
                    *fields.get(*index)?
                }
                (Projection::List(_), ValueShapeDescriptor::List(item)) => *item,
                (Projection::Custom(index), ValueShapeDescriptor::Custom(id)) => {
                    let custom = &self.types.shapes.custom_shapes[id.0];
                    let constructor = match custom.constructor {
                        CustomConstructorRefinement::Exact(constructor) => constructor,
                        CustomConstructorRefinement::Any => {
                            assumptions
                                .iter()
                                .find(|assumption| assumption.path == path[..depth])?
                                .constructor
                        }
                    };
                    let constructor = self.types.customs.types[custom.type_id.index()]
                        .constructors
                        .iter()
                        .find(|candidate| candidate.id.index == constructor)?;
                    let field = constructor.fields.get(*index)?;
                    if let FieldRefinement::Argument(index) = &field.refinement {
                        custom.arguments[*index]
                    } else {
                        field.shape
                    }
                }
                _ => return None,
            };
        }
        Some(shape)
    }

    fn condition(
        &self,
        block: BlockId,
        condition: Condition<'data>,
        source: &Place,
        fact: ConstructorFact,
        assumptions: &[Assumption],
    ) -> Option<Vec<Query>> {
        let Condition::Match { matcher, success } = condition else {
            return None;
        };
        let subject = Place::local(Address::of(&matcher.subject)).normalize(block, self.blocks)?;
        if source.root != subject.root {
            return None;
        }
        let path = source.path.strip_prefix(subject.path.as_slice())?;
        if success {
            return match_proves(place::pattern_at(&matcher.pattern, path)?, true, fact)
                .then(Vec::new);
        }
        let mut requirements = Vec::new();
        let mut parent = subject;
        let mut pattern = &matcher.pattern;
        for projection in path {
            let mut visited = HashSet::new();
            while let MatchPattern::Alias { pattern: inner, .. } = pattern {
                if !visited.insert(pattern as *const MatchPattern) {
                    return None;
                }
                pattern = inner;
            }
            let (index, fields) = match (projection, pattern) {
                (Projection::Tuple(index), MatchPattern::Tuple(fields)) => (index, fields),
                (
                    Projection::Custom(index),
                    MatchPattern::Custom {
                        constructor,
                        fields,
                    },
                ) => {
                    requirements.push(Query {
                        block,
                        place: parent.clone(),
                        fact: ConstructorFact::Is(constructor.index),
                        assumptions: assumptions.to_vec(),
                    });
                    (index, fields)
                }
                _ => return None,
            };
            for (other, field) in fields.iter().enumerate() {
                if other != *index && !irrefutable(field) {
                    return None;
                }
            }
            pattern = fields.get(*index)?;
            parent.path.push(*projection);
        }
        if match_proves(pattern, false, fact) {
            return Some(requirements);
        }
        let (constructor, fields) = custom(pattern)?;
        if fact != ConstructorFact::IsNot(constructor) {
            return None;
        }
        let mut assumptions = assumptions.to_vec();
        let parent_constructor = Assumption {
            path: parent.path.clone(),
            constructor,
        };
        if !assumptions.contains(&parent_constructor) {
            assumptions.push(parent_constructor);
        }
        // A failed C(fields) excludes C only if all fields would have matched
        // under the hypothesis that this same immutable value is C. Discharge
        // those obligations on the predecessor, before the failed match.
        for (index, field) in fields.iter().enumerate() {
            let mut field_place = parent.clone();
            field_place.path.push(Projection::Custom(index));
            pattern_requirements(block, field, field_place, &assumptions, &mut requirements)?;
        }
        Some(requirements)
    }
}

impl Query {
    fn normalize<Graph: ExecutionGraphProfile>(
        mut self,
        blocks: &Blocks<'_, Graph>,
    ) -> Option<Self> {
        let base = Place::local(self.place.root).normalize(self.block, blocks)?;
        self.place.root = base.root;
        self.place.path.splice(..0, base.path.iter().copied());
        for assumption in &mut self.assumptions {
            assumption.path.splice(..0, base.path.iter().copied());
        }
        Some(self)
    }
}

impl ConstructorFact {
    fn holds(self, constructor: usize) -> bool {
        match self {
            Self::Is(expected) => constructor == expected,
            Self::IsNot(excluded) => constructor != excluded,
        }
    }
}

fn match_proves(pattern: &MatchPattern, success: bool, fact: ConstructorFact) -> bool {
    let Some((index, fields)) = custom(pattern) else {
        return false;
    };
    if success {
        return fact.holds(index);
    }
    matches!(fact, ConstructorFact::IsNot(excluded) if excluded == index)
        && fields.iter().all(irrefutable)
}

fn custom(pattern: &MatchPattern) -> Option<(usize, &[MatchPattern])> {
    let mut current = pattern;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current as *const MatchPattern) {
            return None;
        }
        match current {
            MatchPattern::Custom {
                constructor,
                fields,
            } => return Some((constructor.index, fields)),
            MatchPattern::Alias { pattern, .. } => current = pattern,
            _ => return None,
        }
    }
}

fn irrefutable(pattern: &MatchPattern) -> bool {
    let mut pending = vec![(pattern, false)];
    let mut active = HashSet::new();
    let mut complete = HashSet::new();
    while let Some((pattern, leaving)) = pending.pop() {
        let key = pattern as *const MatchPattern;
        if leaving {
            active.remove(&key);
            complete.insert(key);
            continue;
        }
        if complete.contains(&key) {
            continue;
        }
        if !active.insert(key) {
            return false;
        }
        pending.push((pattern, true));
        match pattern {
            MatchPattern::Bind(_) | MatchPattern::Discard => {}
            MatchPattern::Tuple(fields) => {
                pending.extend(fields.iter().map(|field| (field, false)));
            }
            MatchPattern::Alias { pattern, .. } => pending.push((pattern, false)),
            _ => return false,
        }
    }
    true
}

fn pattern_requirements(
    block: BlockId,
    pattern: &MatchPattern,
    place: Place,
    assumptions: &[Assumption],
    requirements: &mut Vec<Query>,
) -> Option<()> {
    let mut pending = vec![PatternVisit::Enter(pattern, place, assumptions.to_vec())];
    let mut active = HashSet::new();
    while let Some(visit) = pending.pop() {
        let (pattern, place, mut assumptions) = match visit {
            PatternVisit::Enter(pattern, place, assumptions) => (pattern, place, assumptions),
            PatternVisit::Leave(key) => {
                active.remove(&key);
                continue;
            }
        };
        let key = pattern as *const MatchPattern;
        if !active.insert(key) {
            return None;
        }
        pending.push(PatternVisit::Leave(key));
        let (fields, custom) = match pattern {
            MatchPattern::Bind(_) | MatchPattern::Discard => continue,
            MatchPattern::Alias { pattern, .. } => {
                pending.push(PatternVisit::Enter(pattern, place, assumptions));
                continue;
            }
            MatchPattern::Tuple(fields) => (fields, false),
            MatchPattern::Custom {
                constructor,
                fields,
            } => {
                requirements.push(Query {
                    block,
                    place: place.clone(),
                    fact: ConstructorFact::Is(constructor.index),
                    assumptions: assumptions.to_vec(),
                });
                assumptions.push(Assumption {
                    path: place.path.clone(),
                    constructor: constructor.index,
                });
                (fields, true)
            }
            _ => return None,
        };
        for (index, field) in fields.iter().enumerate() {
            let mut child = place.clone();
            child.path.push(if custom {
                Projection::Custom(index)
            } else {
                Projection::Tuple(index)
            });
            pending.push(PatternVisit::Enter(field, child, assumptions.clone()));
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::{
        BlockId, Blocks, ConstructorFact, Control, CustomConstructorRefinement, Locals,
        MatchPattern, ParamLocal, ParamSlot, Types, ValueShapeDescriptor, irrefutable,
        match_proves,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, Edge, Jump, Match, MatchEdge, MatchEdgeArgument, MatchPatternBinding,
        ProfiledBlock, ProfiledBlockGraph, Terminator, Transfer,
    };
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::CustomConstructorId;
    use std::convert::Infallible;

    #[test]
    fn conditional_exclusions_reuse_parent_hypotheses_and_reject_unknown_fields() {
        use super::{Assumption, Condition, Place};

        let source = r#"
pub type Option(a) { Some(a) None }
fn inspect(value: Result(Option(Int), String)) -> String {
  case value {
    Ok(Some(_)) -> "present"
    Ok(None) -> "missing"
    Error(reason) -> reason
  }
}
pub fn main() { inspect(Error("failed")) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let graph = plan.program.functions.value_returns.string_functions[1]
            .body()
            .block_graph();
        let matchers = graph
            .blocks
            .iter()
            .filter_map(|block| match &block.terminator {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matchers.len(), 2);
        let mut matcher = matchers[0].clone();
        let blocks = Blocks::admit(graph).unwrap();
        let control = Control {
            blocks: &blocks,
            types: &types,
        };
        let subject = Place::local(super::Address::of(&matcher.subject));
        let parent = Assumption {
            path: Vec::new(),
            constructor: 0,
        };
        let obligations = control
            .condition(
                graph.entry,
                Condition::Match {
                    matcher: &matcher,
                    success: false,
                },
                &subject,
                ConstructorFact::IsNot(0),
                std::slice::from_ref(&parent),
            )
            .unwrap();
        assert_eq!(obligations.len(), 1);
        assert!(obligations[0].fact == ConstructorFact::Is(0));
        assert_eq!(obligations[0].assumptions.len(), 1);
        assert!(obligations[0].assumptions[0] == parent);

        for pattern in [
            MatchPattern::Bool(true),
            MatchPattern::Custom {
                constructor: CustomConstructorId {
                    type_id: crate::plan::execution::type_::CustomTypeId(0),
                    index: 0,
                },
                fields: vec![MatchPattern::Bool(true)].into(),
            },
        ] {
            matcher.pattern = pattern;
            assert!(
                control
                    .condition(
                        graph.entry,
                        Condition::Match {
                            matcher: &matcher,
                            success: false
                        },
                        &subject,
                        ConstructorFact::IsNot(0),
                        &[],
                    )
                    .is_none()
            );
        }
    }

    #[test]
    fn nested_pattern_obligations_keep_parent_hypotheses_and_reject_unknown_matches() {
        use super::{Assumption, Place, Projection, pattern_requirements};
        use crate::plan::execution::graph::TupleLocalId;
        use crate::plan::execution::type_::CustomTypeId;

        let root = Place::local(TupleLocalId(0).into());
        let parent = Assumption {
            path: Vec::new(),
            constructor: 7,
        };
        let leaf = MatchPattern::Custom {
            constructor: CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 2,
            },
            fields: vec![MatchPattern::Bind(MatchPatternBinding::new(0))].into(),
        };
        let nested = MatchPattern::Custom {
            constructor: CustomConstructorId {
                type_id: CustomTypeId(1),
                index: 3,
            },
            fields: vec![leaf].into(),
        };
        static SHARED: MatchPattern = MatchPattern::Discard;
        let pattern = MatchPattern::Tuple(
            vec![
                MatchPattern::Alias {
                    pattern: Box::new(nested).into(),
                    binding: MatchPatternBinding::new(1),
                },
                MatchPattern::Alias {
                    pattern: Node::Static(&SHARED),
                    binding: MatchPatternBinding::new(2),
                },
                MatchPattern::Alias {
                    pattern: Node::Static(&SHARED),
                    binding: MatchPatternBinding::new(3),
                },
            ]
            .into(),
        );
        let mut obligations = Vec::new();
        assert_eq!(
            pattern_requirements(
                BlockId(4),
                &pattern,
                root.clone(),
                std::slice::from_ref(&parent),
                &mut obligations
            ),
            Some(())
        );
        assert_eq!(obligations.len(), 2);
        for (query, constructor, path, assumed) in [
            (
                &obligations[0],
                3,
                vec![Projection::Tuple(0)],
                vec![(Vec::new(), 7)],
            ),
            (
                &obligations[1],
                2,
                vec![Projection::Tuple(0), Projection::Custom(0)],
                vec![(Vec::new(), 7), (vec![Projection::Tuple(0)], 3)],
            ),
        ] {
            assert_eq!(query.block, BlockId(4));
            assert_eq!(
                query.place,
                Place {
                    root: root.root,
                    path
                }
            );
            assert!(query.fact == ConstructorFact::Is(constructor));
            assert_eq!(
                query
                    .assumptions
                    .iter()
                    .map(|a| (a.path.clone(), a.constructor))
                    .collect::<Vec<_>>(),
                assumed
            );
        }
        static CYCLE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&CYCLE),
            binding: MatchPatternBinding { index: 0 },
        };
        for unknown in [&MatchPattern::Bool(true), &CYCLE] {
            assert_eq!(
                pattern_requirements(
                    BlockId(4),
                    unknown,
                    root.clone(),
                    std::slice::from_ref(&parent),
                    &mut Vec::new()
                ),
                None
            );
        }
    }

    #[test]
    fn nested_exclusions_require_every_incoming_path_and_survive_unchanged_loops() {
        let source = r#"
pub type Option(a) { Some(a) None }
fn inspect(value: Result(Option(Int), String)) -> String {
  case value {
    Ok(Some(_)) -> "present"
    Ok(None) -> "missing"
    Error(reason) -> reason
  }
}
pub fn main() { inspect(Error("failed")) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let graph = plan.program.functions.value_returns.string_functions[1]
            .body()
            .block_graph();
        assert_eq!(graph.blocks.len(), 5);
        let source = &graph.block(BlockId(4)).params()[0].local;
        for (bypass, cycle, expected) in [
            (false, false, true),
            (true, false, false),
            (false, true, true),
            (true, true, false),
        ] {
            let mut bodies = Vec::new();
            for (index, block) in graph.blocks().enumerate() {
                let mut terminator = block.terminator().clone();
                if let Terminator::Match(matcher) = &mut terminator
                    && index == 0
                    && bypass
                {
                    matcher.success.target = BlockId(4);
                    matcher.success.args =
                        vec![MatchEdgeArgument::Value(matcher.subject.clone())].into();
                }
                if index == 4 && cycle {
                    terminator = Terminator::Jump(Jump {
                        edge: Edge::new(
                            BlockId(4),
                            vec![source.clone()],
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    });
                }
                bodies.push(ProfiledBlock::new(
                    block.params().to_vec(),
                    block.instructions().to_vec(),
                    terminator,
                ));
            }
            let raw: ProfiledBlockGraph<Infallible> =
                ProfiledBlockGraph::from_parts(graph.entry, bodies);
            let blocks = Blocks::admit(&raw).unwrap();
            let control = Control {
                blocks: &blocks,
                types: &types,
            };
            assert_eq!(
                control.proves(BlockId(4), source, ConstructorFact::IsNot(0)),
                expected
            );
            assert_eq!(
                control.proves(BlockId(4), source, ConstructorFact::Is(1)),
                expected
            );
        }
    }

    #[test]
    fn requires_every_incoming_path_to_establish_a_constructor() {
        let typed = crate::compile_typed_module("example", "src/example.gleam", "pub type Choice { First(Int) Second(Int) } fn read(base: Int, x) { case x { First(n) -> base + n Second(n) -> base + n } } pub fn main() { read(0, First(42)) }").unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let (slot, local) = plan.program.functions.value_returns.int_functions.iter()
            .flat_map(|function| function.body().block_graph().blocks())
            .flat_map(|block| block.params())
            .filter_map(|slot| match &slot.local {
                ParamLocal::Custom(local) => Some((slot, local)),
                _ => None,
            })
            .find(|(slot, _)| matches!(types.shape(slot.shape), Ok(ValueShapeDescriptor::Custom(id)) if types.custom_shape_descriptor(*id).unwrap().constructor == CustomConstructorRefinement::Any))
            .unwrap();
        for (bypass, cycle) in [(false, false), (true, false), (false, true), (true, true)] {
            let graph = branch_graph(
                slot,
                CustomConstructorId {
                    type_id: local.shape.type_id,
                    index: 0,
                },
                bypass,
                cycle,
            );
            let blocks = Blocks::admit(&graph).unwrap();
            let control = Control {
                blocks: &blocks,
                types: &types,
            };
            let parameter = &blocks.block(BlockId(1)).unwrap().params()[0];
            assert_eq!(
                control.proves(BlockId(1), &parameter.local, ConstructorFact::Is(0)),
                !bypass
            );
            assert_eq!(
                control.proves(BlockId(1), &parameter.local, ConstructorFact::IsNot(1)),
                !bypass
            );
            assert!(!control.proves(BlockId(0), &slot.local, ConstructorFact::Is(0)));
            assert!(!control.proves(BlockId(2), &slot.local, ConstructorFact::Is(1)));
            assert!(!control.proves(BlockId(99), &slot.local, ConstructorFact::Is(0)));
            assert!(!control.proves(
                BlockId(1),
                &ParamLocal::Int(crate::plan::execution::graph::IntLocalId(99)),
                ConstructorFact::Is(0)
            ));
            let mut locals = Locals::default();
            locals.define(parameter, &types).unwrap();
            control.refine(BlockId(1), parameter, &mut locals);
            assert_eq!(locals.allows_constructor(&parameter.local, 1), bypass);
            assert!(locals.allows_constructor(&parameter.local, 0));
        }
    }

    #[test]
    fn malformed_predecessors_and_projection_cycles_cannot_prove_a_constructor() {
        use crate::plan::execution::graph::{
            CustomInstruction, CustomLocal, CustomLocalId, ProfiledInstruction,
            ProfiledInstructionKind,
        };
        use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Choice { First(Int) Second(Int) }
fn widen(value: Choice) { value }
pub fn main() { #(widen(First(42)), Second(7)) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let body = plan.program.functions.value_returns.custom_functions[0].body();
        let slot = &body.function_body().block_graph().params[0];
        let nominal = common
            .value_shapes
            .custom_shapes
            .iter()
            .position(|shape| shape.constructor == CustomConstructorRefinement::Any)
            .unwrap();
        let local = CustomLocal::new(
            CustomLocalId(0),
            CustomValueShape::new(CustomTypeId(0), CustomValueShapeId(nominal)),
        );
        assert_eq!(slot.local, ParamLocal::Custom(local));

        enum Corruption {
            MissingArgument,
            MissingBinding,
            SourceCycle,
            TargetCycle,
            GrowingBackEdge,
            UnprovenBackEdge,
        }
        for corruption in [
            Corruption::MissingArgument,
            Corruption::MissingBinding,
            Corruption::SourceCycle,
            Corruption::TargetCycle,
            Corruption::GrowingBackEdge,
            Corruption::UnprovenBackEdge,
        ] {
            let mut source_instructions = Vec::new();
            let mut target_instructions = Vec::new();
            let mut source_exit = Terminator::Jump(Jump {
                edge: Edge::new(
                    BlockId(1),
                    vec![slot.local.clone()],
                    Transfer {
                        families: Table::Static(&[]),
                    },
                ),
            });
            let mut target_exit = Terminator::Exit(BlockGraphExitId(0));
            match corruption {
                Corruption::MissingArgument => {
                    source_exit = Terminator::Jump(Jump {
                        edge: Edge::new(
                            BlockId(1),
                            Vec::new(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    })
                }
                Corruption::MissingBinding => {
                    source_exit = Terminator::Match(Match {
                        subject: slot.local.clone(),
                        pattern: MatchPattern::Discard,
                        success: MatchEdge::new(
                            BlockId(1),
                            vec![MatchEdgeArgument::Binding(99)],
                            Vec::new(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                        failure: Edge::new(
                            BlockId(2),
                            Vec::new(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    })
                }
                Corruption::SourceCycle | Corruption::TargetCycle => {
                    let instruction = ProfiledInstruction::new(
                        slot.clone(),
                        ProfiledInstructionKind::Custom(CustomInstruction::CustomField {
                            source: local,
                            index: 0,
                        }),
                    );
                    match corruption {
                        Corruption::SourceCycle => source_instructions.push(instruction),
                        _ => target_instructions.push(instruction),
                    }
                }
                Corruption::GrowingBackEdge => {
                    let projected = CustomLocal::new(CustomLocalId(1), local.shape);
                    target_instructions.push(ProfiledInstruction::new(
                        ParamSlot::new(ParamLocal::Custom(projected), slot.shape),
                        ProfiledInstructionKind::Custom(CustomInstruction::CustomField {
                            source: local,
                            index: 0,
                        }),
                    ));
                    target_exit = Terminator::Jump(Jump {
                        edge: Edge::new(
                            BlockId(1),
                            vec![ParamLocal::Custom(projected)],
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    });
                }
                Corruption::UnprovenBackEdge => {
                    target_exit = Terminator::Jump(Jump {
                        edge: Edge::new(
                            BlockId(1),
                            vec![slot.local.clone()],
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    })
                }
            }
            let graph: ProfiledBlockGraph<Infallible> = ProfiledBlockGraph::from_parts(
                BlockId(0),
                vec![
                    ProfiledBlock::new(vec![slot.clone()], source_instructions, source_exit),
                    ProfiledBlock::new(vec![slot.clone()], target_instructions, target_exit),
                    ProfiledBlock::new(
                        Vec::new(),
                        Vec::new(),
                        Terminator::Exit(BlockGraphExitId(1)),
                    ),
                ],
            );
            let blocks = Blocks::admit(&graph).unwrap();
            let control = Control {
                blocks: &blocks,
                types: &types,
            };
            assert!(!control.proves(BlockId(1), &slot.local, ConstructorFact::IsNot(1)));
        }
    }

    #[test]
    fn converging_paths_reuse_the_same_established_constructor_fact() {
        use crate::plan::execution::graph::{BoolBranch, BoolLocalId};
        use crate::plan::execution::type_::{ValueShapeId, ValueType};
        let source = "pub type Choice { First(Int) Second(Int) } fn widen(x: Choice) { x } pub fn main() { #(widen(First(42)), Second(7), True) }";
        let module = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let slot = &plan.program.functions.value_returns.custom_functions[0]
            .body()
            .function_body()
            .block_graph()
            .params[0];
        let boolean = ParamSlot::new(
            ParamLocal::Bool(BoolLocalId(0)),
            ValueShapeId(
                types
                    .shape_types()
                    .iter()
                    .position(|type_| type_ == &ValueType::Bool)
                    .unwrap(),
            ),
        );
        let parameters = vec![slot.clone(), boolean];
        let arguments = parameters
            .iter()
            .map(|slot| slot.local.clone())
            .collect::<Vec<_>>();
        let graph: ProfiledBlockGraph<Infallible> = ProfiledBlockGraph::from_parts(
            BlockId(0),
            vec![
                ProfiledBlock::new(
                    parameters.clone(),
                    vec![],
                    Terminator::Match(Match {
                        subject: slot.local.clone(),
                        pattern: MatchPattern::Custom {
                            constructor: CustomConstructorId {
                                type_id: crate::plan::execution::type_::CustomTypeId(0),
                                index: 1,
                            },
                            fields: vec![MatchPattern::Discard].into(),
                        },
                        success: MatchEdge::new(
                            BlockId(5),
                            vec![],
                            Vec::new(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                        failure: Edge::new(
                            BlockId(1),
                            arguments.clone(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    }),
                ),
                ProfiledBlock::new(
                    parameters.clone(),
                    vec![],
                    Terminator::BoolBranch(BoolBranch {
                        subject: BoolLocalId(0),
                        true_: Edge::new(
                            BlockId(2),
                            arguments.clone(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                        false_: Edge::new(
                            BlockId(3),
                            arguments.clone(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    }),
                ),
                ProfiledBlock::new(
                    parameters.clone(),
                    vec![],
                    Terminator::Jump(Jump {
                        edge: Edge::new(
                            BlockId(4),
                            arguments.clone(),
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    }),
                ),
                ProfiledBlock::new(
                    parameters.clone(),
                    vec![],
                    Terminator::Jump(Jump {
                        edge: Edge::new(
                            BlockId(4),
                            arguments,
                            Transfer {
                                families: Table::Static(&[]),
                            },
                        ),
                    }),
                ),
                ProfiledBlock::new(parameters, vec![], Terminator::Exit(BlockGraphExitId(0))),
                ProfiledBlock::new(vec![], vec![], Terminator::Exit(BlockGraphExitId(1))),
            ],
        );
        let blocks = Blocks::admit(&graph).unwrap();
        let control = Control {
            blocks: &blocks,
            types: &types,
        };
        assert!(control.proves(BlockId(4), &slot.local, ConstructorFact::IsNot(1)));
        assert!(control.proves(BlockId(4), &slot.local, ConstructorFact::Is(0)));
        assert!(!control.proves(BlockId(0), &slot.local, ConstructorFact::Is(0)));
    }

    fn branch_graph(
        slot: &ParamSlot,
        constructor: CustomConstructorId,
        bypass: bool,
        cycle: bool,
    ) -> ProfiledBlockGraph<Infallible> {
        let jump = || {
            Terminator::Jump(Jump {
                edge: Edge {
                    target: BlockId(1),
                    args: vec![slot.local.clone()].into(),
                    transfer: Transfer {
                        families: Table::Static(&[]),
                    },
                },
            })
        };
        let exit = || Terminator::Exit(BlockGraphExitId(0));
        ProfiledBlockGraph::from_parts(
            BlockId(0),
            vec![
                ProfiledBlock::new(
                    vec![slot.clone()],
                    vec![],
                    Terminator::Match(Match {
                        subject: slot.local.clone(),
                        pattern: MatchPattern::Alias {
                            pattern: Node::Owned(Box::new(MatchPattern::Custom {
                                constructor,
                                fields: vec![MatchPattern::Discard].into(),
                            })),
                            binding: MatchPatternBinding { index: 0 },
                        },
                        success: MatchEdge {
                            target: BlockId(1),
                            args: vec![MatchEdgeArgument::Binding(0)].into(),
                            bindings: Vec::new().into(),
                            transfer: Transfer {
                                families: Table::Static(&[]),
                            },
                        },
                        failure: Edge {
                            target: BlockId(2),
                            args: vec![slot.local.clone()].into(),
                            transfer: Transfer {
                                families: Table::Static(&[]),
                            },
                        },
                    }),
                ),
                ProfiledBlock::new(
                    vec![slot.clone()],
                    vec![],
                    if cycle { jump() } else { exit() },
                ),
                ProfiledBlock::new(
                    vec![slot.clone()],
                    vec![],
                    if bypass { jump() } else { exit() },
                ),
            ],
        )
    }

    #[test]
    fn shared_irrefutable_nodes_are_not_recursive_patterns() {
        static DISCARD: MatchPattern = MatchPattern::Discard;
        static SHARED: MatchPattern = MatchPattern::Tuple(Table::Static(&[
            MatchPattern::Alias {
                pattern: Node::Static(&DISCARD),
                binding: MatchPatternBinding { index: 0 },
            },
            MatchPattern::Alias {
                pattern: Node::Static(&DISCARD),
                binding: MatchPatternBinding { index: 1 },
            },
        ]));
        static RECURSIVE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&RECURSIVE),
            binding: MatchPatternBinding { index: 0 },
        };
        assert!(irrefutable(&SHARED));
        assert!(!irrefutable(&RECURSIVE));
        for (field, excluded) in [(&SHARED, true), (&RECURSIVE, false)] {
            let pattern = MatchPattern::Custom {
                constructor: CustomConstructorId {
                    type_id: crate::plan::execution::type_::CustomTypeId(0),
                    index: 7,
                },
                fields: vec![MatchPattern::Alias {
                    pattern: Node::Static(field),
                    binding: MatchPatternBinding { index: 2 },
                }]
                .into(),
            };
            assert_eq!(
                match_proves(&pattern, false, ConstructorFact::IsNot(7)),
                excluded
            );
        }
    }

    #[test]
    fn projected_shapes_preserve_generic_arguments_and_require_exact_constructors() {
        use super::{Projection, ValueShapeId};
        use crate::plan::execution::graph::CustomLocal;

        let source = r#"
pub type Choice { First(Int) Second(Int) }
pub type Wrap(a) { Wrap(a) }
pub type Fixed { Fixed(Int) }
fn widen(value: Choice) { value }
pub fn main() { #(Wrap(First(42)), Fixed(42), [widen(First(42))], #(First(42))) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let body = plan.program.functions.value_returns.tuple_functions[0].body();
        let blocks = Blocks::admit(body.block_graph()).unwrap();
        let control = Control {
            blocks: &blocks,
            types: &types,
        };
        let root = body.block_graph().instructions.iter().find(|instruction| {
            matches!(&instruction.output.local, ParamLocal::Tuple { type_, .. } if type_.len() == 4)
        }).unwrap().output.shape;
        let int = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| matches!(shape, ValueShapeDescriptor::Int))
                .unwrap(),
        );
        let first = body
            .block_graph()
            .instructions
            .iter()
            .find(|instruction| {
                matches!(&instruction.output.local, ParamLocal::Custom(CustomLocal { shape, .. })
                if common.custom_types.types[shape.type_id.index()].type_.name.as_str() == "Choice")
            })
            .unwrap()
            .output
            .shape;
        for (path, expected) in [
            (
                vec![Projection::Tuple(0), Projection::Custom(0)],
                Some(first),
            ),
            (vec![Projection::Tuple(1), Projection::Custom(0)], Some(int)),
            (
                vec![Projection::Tuple(3), Projection::Tuple(0)],
                Some(first),
            ),
            (
                vec![
                    Projection::Tuple(2),
                    Projection::List(0),
                    Projection::Custom(0),
                ],
                None,
            ),
            (vec![Projection::Tuple(0), Projection::Custom(99)], None),
            (vec![Projection::Tuple(99)], None),
            (vec![Projection::Custom(0)], None),
        ] {
            assert_eq!(control.projected_shape(root, &path, &[]), expected);
        }
        assert_eq!(
            control.projected_shape(ValueShapeId(99_999), &[Projection::Tuple(0)], &[]),
            None
        );
        let item = control
            .projected_shape(root, &[Projection::Tuple(2), Projection::List(99)], &[])
            .unwrap();
        assert_eq!(
            types.shape_type(item).unwrap(),
            types.shape_type(first).unwrap()
        );
        assert!(types.is_nominal_shape(item));
        assert!(!types.is_nominal_shape(first));

        // A type can retain an unmaterialized constructor, but it has no fields to project.
        let mut custom_shapes = common.value_shapes.custom_shapes.to_vec();
        let mut missing = custom_shapes
            .iter()
            .find(|shape| {
                types.shape_type(first).unwrap()
                    == &crate::plan::execution::type_::ValueType::Custom(shape.type_id)
            })
            .unwrap()
            .clone();
        missing.constructor = CustomConstructorRefinement::Exact(1);
        let missing_custom = crate::plan::execution::type_::CustomValueShapeId(custom_shapes.len());
        custom_shapes.push(missing);
        let missing_shape = ValueShapeId(common.value_shapes.shapes.len());
        let mut shapes = common.value_shapes.shapes.to_vec();
        shapes.push(ValueShapeDescriptor::Custom(missing_custom));
        let mut shape_types = common.value_shapes.shape_types.to_vec();
        shape_types.push(types.shape_type(first).unwrap().clone());
        let raw = crate::plan::execution::type_::ValueShapeTable {
            shapes: shapes.into(),
            shape_types: shape_types.into(),
            custom_shapes: custom_shapes.into(),
        };
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &raw,
        )
        .unwrap();
        let control = Control {
            blocks: &blocks,
            types: &types,
        };
        assert_eq!(
            control.projected_shape(missing_shape, &[Projection::Custom(0)], &[]),
            None
        );
    }

    #[test]
    fn failed_nested_patterns_require_the_parent_and_irrefutable_siblings() {
        use super::{Condition, Place, Projection};
        use crate::plan::execution::graph::TupleLocalId;
        use crate::plan::execution::type_::{CustomTypeId, ValueShapeId};

        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 42 }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let graph = ProfiledBlockGraph::<Infallible>::from_parts(
            BlockId(0),
            vec![ProfiledBlock::new(
                vec![ParamSlot {
                    local: ParamLocal::Tuple {
                        local: TupleLocalId(0),
                        type_: Vec::new().into(),
                    },
                    shape: ValueShapeId(0),
                }],
                Vec::new(),
                Terminator::Exit(BlockGraphExitId(0)),
            )],
        );
        let blocks = Blocks::admit(&graph).unwrap();
        let control = Control {
            blocks: &blocks,
            types: &types,
        };
        let leaf = MatchPattern::Custom {
            constructor: CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 3,
            },
            fields: Vec::new().into(),
        };
        for (custom_parent, sibling, expected) in [
            (false, MatchPattern::Discard, true),
            (false, MatchPattern::Bool(true), false),
            (true, MatchPattern::Discard, true),
            (true, MatchPattern::Bool(true), false),
        ] {
            let fields = vec![leaf.clone(), sibling].into();
            let pattern = if custom_parent {
                MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        type_id: CustomTypeId(1),
                        index: 7,
                    },
                    fields,
                }
            } else {
                MatchPattern::Tuple(fields)
            };
            let matcher = Match {
                subject: ParamLocal::Tuple {
                    local: TupleLocalId(0),
                    type_: Vec::new().into(),
                },
                pattern: MatchPattern::Alias {
                    pattern: Box::new(pattern).into(),
                    binding: MatchPatternBinding::new(0),
                },
                success: MatchEdge::new(
                    BlockId(0),
                    Vec::new(),
                    Vec::new(),
                    Transfer {
                        families: Table::Static(&[]),
                    },
                ),
                failure: Edge::new(
                    BlockId(0),
                    Vec::new(),
                    Transfer {
                        families: Table::Static(&[]),
                    },
                ),
            };
            let path = vec![if custom_parent {
                Projection::Custom(0)
            } else {
                Projection::Tuple(0)
            }];
            let source = Place {
                root: TupleLocalId(0).into(),
                path: path.clone(),
            };
            let requirements = control.condition(
                BlockId(0),
                Condition::Match {
                    matcher: &matcher,
                    success: false,
                },
                &source,
                ConstructorFact::IsNot(3),
                &[],
            );
            assert_eq!(requirements.is_some(), expected);
            if let Some(requirements) = requirements {
                assert_eq!(requirements.len(), usize::from(custom_parent));
                for query in requirements {
                    assert_eq!(query.block, BlockId(0));
                    assert_eq!(query.place, Place::local(TupleLocalId(0).into()));
                    assert!(query.fact == ConstructorFact::Is(7));
                }
            }
            assert!(
                control
                    .condition(
                        BlockId(0),
                        Condition::Match {
                            matcher: &matcher,
                            success: true
                        },
                        &source,
                        ConstructorFact::Is(3),
                        &[],
                    )
                    .is_some()
            );
            for (block, root, path, success) in [
                (99, 0, path.clone(), false),
                (0, 1, path, false),
                (0, 0, vec![Projection::List(0)], false),
                (0, 0, vec![Projection::Tuple(99)], false),
                (0, 0, vec![Projection::Tuple(99)], true),
            ] {
                assert!(
                    control
                        .condition(
                            BlockId(block),
                            Condition::Match {
                                matcher: &matcher,
                                success
                            },
                            &Place {
                                root: TupleLocalId(root).into(),
                                path
                            },
                            ConstructorFact::IsNot(3),
                            &[],
                        )
                        .is_none()
                );
            }
        }
        static CYCLE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&CYCLE),
            binding: MatchPatternBinding { index: 0 },
        };
        let matcher = Match {
            subject: ParamLocal::Tuple {
                local: TupleLocalId(0),
                type_: Vec::new().into(),
            },
            pattern: MatchPattern::Alias {
                pattern: Node::Static(&CYCLE),
                binding: MatchPatternBinding { index: 1 },
            },
            success: MatchEdge::new(
                BlockId(0),
                Vec::new(),
                Vec::new(),
                Transfer {
                    families: Table::Static(&[]),
                },
            ),
            failure: Edge::new(
                BlockId(0),
                Vec::new(),
                Transfer {
                    families: Table::Static(&[]),
                },
            ),
        };
        assert!(
            control
                .condition(
                    BlockId(0),
                    Condition::Match {
                        matcher: &matcher,
                        success: false
                    },
                    &Place {
                        root: TupleLocalId(0).into(),
                        path: vec![Projection::Tuple(0)]
                    },
                    ConstructorFact::IsNot(3),
                    &[],
                )
                .is_none()
        );
    }

    #[test]
    fn failed_field_tests_do_not_exclude_the_outer_constructor() {
        let constructor = CustomConstructorId {
            type_id: crate::plan::execution::type_::CustomTypeId(0),
            index: 7,
        };
        for (field, excluded) in [
            (MatchPattern::Discard, true),
            (MatchPattern::Bind(MatchPatternBinding { index: 0 }), true),
            (
                MatchPattern::Tuple(vec![MatchPattern::Discard, MatchPattern::Discard].into()),
                true,
            ),
            (MatchPattern::Bool(true), false),
            (
                MatchPattern::Int(crate::plan::execution::graph::IntegerLiteral::from(
                    num_bigint::BigInt::from(42),
                )),
                false,
            ),
        ] {
            let pattern = MatchPattern::Custom {
                constructor,
                fields: vec![field].into(),
            };
            assert!(match_proves(&pattern, true, ConstructorFact::Is(7)));
            assert!(match_proves(&pattern, true, ConstructorFact::IsNot(0)));
            assert!(!match_proves(&pattern, true, ConstructorFact::Is(0)));
            assert_eq!(
                match_proves(&pattern, false, ConstructorFact::IsNot(7)),
                excluded
            );
            assert!(!match_proves(&pattern, false, ConstructorFact::Is(0)));
            assert!(!match_proves(&pattern, false, ConstructorFact::IsNot(0)));
        }
        static RECURSIVE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&RECURSIVE),
            binding: MatchPatternBinding { index: 0 },
        };
        assert!(!match_proves(&RECURSIVE, true, ConstructorFact::Is(0)));
        assert!(!match_proves(
            &MatchPattern::Discard,
            false,
            ConstructorFact::IsNot(0)
        ));
    }
}
