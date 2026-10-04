use super::block::Blocks;
use super::guard::Guards;
use super::incoming::{self, Condition, Input};
use super::local::{Address, Locals};
use super::place::{self, Place, Projection};
use super::type_::Types;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BlockId, BoolInstruction, BoolLocalId, BoolTest, MatchPattern, ParamLocal, ParamSlot,
    ProfiledInstructionKind, Terminator,
};
use crate::plan::execution::type_::custom::FieldRefinement;
use crate::plan::execution::type_::{
    CustomConstructorRefinement, ValueShapeDescriptor, ValueShapeId,
};
use std::collections::{HashMap, HashSet};

pub(super) struct Control<'graph, 'data, Graph: ExecutionGraphProfile> {
    pub(super) blocks: &'graph Blocks<'data, Graph>,
    pub(super) types: &'graph Types<'data>,
    pub(super) guards: Guards<'graph, 'data, Graph>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Fact {
    Is(usize),
    IsNot(usize),
    Boolean(bool),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Query {
    block: BlockId,
    place: Place,
    fact: Fact,
    assumptions: Vec<Assumption>,
}

// Proven ancestor constructors and nested pattern hypotheses are relative to
// the query's immutable root while proving constructor facts and exclusions.
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
    pub(super) fn new<'graph>(
        blocks: &'graph Blocks<'data, Graph>,
        types: &'graph Types<'data>,
    ) -> Control<'graph, 'data, Graph> {
        let mut control = Control {
            blocks,
            types,
            guards: Guards::new(blocks),
        };
        // Only previously established exclusions participate in each proof.
        // Share the finite closure with length/prefix checks in this body.
        loop {
            let mut changed = false;
            for (index, block) in blocks.iter().enumerate() {
                for truth in [false, true] {
                    let condition = match block.terminator() {
                        Terminator::Match(matcher) => Condition::Match {
                            matcher,
                            success: truth,
                        },
                        Terminator::BoolBranch(branch) => Condition::Bool {
                            subject: branch.subject,
                            truth,
                        },
                        Terminator::TestBranch(branch) => Condition::Test {
                            test: &branch.test,
                            truth,
                        },
                        _ => continue,
                    };
                    let id = BlockId(index);
                    if !control.guards.excludes(id, condition) && control.contradicts(id, condition)
                    {
                        changed |= control.guards.exclude(id, truth);
                    }
                }
            }
            if !changed {
                return control;
            }
        }
    }

    fn contradicts(&self, block: BlockId, condition: Condition<'data>) -> bool {
        if self.guards.contradicts(block, condition) {
            return true;
        }
        if let Some((subject, value)) = self.boolean_condition(block, condition) {
            return self.proves(block, &ParamLocal::Bool(subject), Fact::Boolean(!value));
        }
        match condition {
            Condition::Match { matcher, success } => {
                if success {
                    let fact = match unaliased(&matcher.pattern) {
                        Some(MatchPattern::Custom { constructor, .. }) => {
                            Fact::IsNot(constructor.index)
                        }
                        Some(MatchPattern::Bool(value)) => Fact::Boolean(!value),
                        _ => return false,
                    };
                    return self.proves(block, &matcher.subject, fact);
                }
                let mut requirements = Vec::new();
                if pattern_requirements(
                    block,
                    &matcher.pattern,
                    Place::local(Address::of(&matcher.subject)),
                    &[],
                    &mut requirements,
                )
                .is_none()
                {
                    return false;
                }
                requirements
                    .into_iter()
                    .all(|query| self.proves_query(query))
            }
            _ => false,
        }
    }

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
        let Some(place) = Place::local(Address::of(&slot.local)).normalize(block, self.blocks)
        else {
            return;
        };
        // A later branch can establish the parent after earlier nested tests.
        // Carry only independently proved, strict ancestors of this same value
        // when those earlier failures are queried for the selected field.
        let assumptions = locals
            .known_constructors()
            .filter_map(|(parent, constructor)| {
                (parent.root == place.root
                    && parent.path.len() < place.path.len()
                    && place.path.starts_with(&parent.path))
                .then_some(Assumption {
                    path: parent.path.clone(),
                    constructor,
                })
            })
            .collect::<Vec<_>>();
        let proves = |fact| {
            self.proves_query(Query {
                block,
                place: place.clone(),
                fact,
                assumptions: assumptions.clone(),
            })
        };
        let mut possible = Vec::new();
        for constructor in self.types.inhabited_constructors(shape) {
            let index = constructor.id.index;
            if proves(Fact::Is(index)) {
                locals.set_constructor(&slot.local, place.clone(), index);
                locals.restrict_constructors(&slot.local, vec![index]);
                return;
            }
            if !proves(Fact::IsNot(index)) {
                possible.push(index);
            }
        }
        if possible.len() == 1 && self.types.has_all_inhabited_constructors(shape) {
            locals.set_constructor(&slot.local, place, possible[0]);
        }
        if !possible.is_empty() && possible.len() != type_.constructors.len() {
            locals.restrict_constructors(&slot.local, possible);
        }
    }

    // Every incoming path must establish the fact. Constructor exclusions prove
    // an exact variant only when every inhabited declared variant is represented.
    fn proves(&self, block: BlockId, local: &ParamLocal, fact: Fact) -> bool {
        self.proves_query(Query {
            block,
            place: Place::local(Address::of(local)),
            fact,
            assumptions: Vec::new(),
        })
    }

    fn proves_query(&self, query: Query) -> bool {
        let mut pending = vec![Visit::Enter(query)];
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
                    .flat_map(|instruction| instruction.outputs())
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
                if let Fact::Is(index) = query.fact
                    && self.types.has_all_inhabited_constructors(shape)
                    && self
                        .types
                        .inhabited_constructors(shape)
                        .any(|constructor| constructor.id.index == index)
                {
                    pending.extend(
                        self.types
                            .inhabited_constructors(shape)
                            .filter(|constructor| constructor.id.index != index)
                            .map(|constructor| {
                                Visit::Enter(Query {
                                    fact: Fact::IsNot(constructor.id.index),
                                    ..query.clone()
                                })
                            }),
                    );
                    continue;
                }
            }
            if let Fact::Boolean(expected) = query.fact
                && query.place.path.is_empty()
                && let Some(instruction) = block
                    .instructions()
                    .iter()
                    .filter_map(|instruction| instruction.value())
                    .find(|instruction| Address::of(&instruction.output.local) == query.place.root)
                && let ProfiledInstructionKind::Bool(instruction) = &instruction.kind
            {
                match instruction {
                    BoolInstruction::Value(value) => {
                        if *value != expected {
                            return false;
                        }
                        continue;
                    }
                    BoolInstruction::Test(test) => {
                        let Some((subject, value)) = self.boolean_condition(
                            query.block,
                            Condition::Test {
                                test,
                                truth: expected,
                            },
                        ) else {
                            return false;
                        };
                        pending.push(Visit::Enter(Query {
                            place: Place::local(Address::from(subject)),
                            fact: Fact::Boolean(value),
                            ..query
                        }));
                        continue;
                    }
                    _ => return false,
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
                if self.guards.excludes(input.block, input.condition) {
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
        fact: Fact,
        assumptions: &[Assumption],
    ) -> Option<Vec<Query>> {
        if let Fact::Boolean(expected) = fact
            && let Some((subject, value)) = self.boolean_condition(block, condition)
        {
            return Place::local(Address::from(subject))
                .normalize(block, self.blocks)
                .is_some_and(|place| place == *source && value == expected)
                .then(Vec::new);
        }
        self.match_condition(block, condition, source, fact, assumptions)
    }

    fn boolean_condition(
        &self,
        block: BlockId,
        condition: Condition<'data>,
    ) -> Option<(BoolLocalId, bool)> {
        let (test, truth) = match condition {
            Condition::Bool { subject, truth } => return Some((subject, truth)),
            Condition::Test { test, truth } => (test, truth),
            _ => return None,
        };
        match test {
            BoolTest::Not(subject) => Some((*subject, !truth)),
            BoolTest::Equal {
                left: ParamLocal::Bool(left),
                right: ParamLocal::Bool(right),
            } => {
                let block = self.blocks.find_block(block)?;
                let literal = |local: BoolLocalId| {
                    block
                        .instructions()
                        .iter()
                        .filter_map(|instruction| instruction.value())
                        .find(|instruction| {
                            Address::of(&instruction.output.local) == Address::from(local)
                        })
                        .and_then(|instruction| match &instruction.kind {
                            ProfiledInstructionKind::Bool(BoolInstruction::Value(value)) => {
                                Some(*value)
                            }
                            _ => None,
                        })
                };
                if let Some(value) = literal(*left) {
                    Some((*right, value == truth))
                } else {
                    literal(*right).map(|value| (*left, value == truth))
                }
            }
            _ => None,
        }
    }

    pub(super) fn failed_match_pattern<'pattern>(
        &self,
        block: BlockId,
        pattern: &'pattern MatchPattern,
        subject: Place,
        path: &[Projection],
    ) -> Option<&'pattern MatchPattern> {
        let (pattern, _, requirements) = failure_path(block, pattern, subject, path, &[])?;
        requirements
            .into_iter()
            .all(|query| self.proves_query(query))
            .then_some(pattern)
    }

    fn match_condition(
        &self,
        block: BlockId,
        condition: Condition<'data>,
        source: &Place,
        fact: Fact,
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
        let (pattern, parent, mut requirements) =
            failure_path(block, &matcher.pattern, subject, path, assumptions)?;
        if match_proves(pattern, false, fact) {
            return Some(requirements);
        }
        let (constructor, fields) = custom(pattern)?;
        if fact != Fact::IsNot(constructor) {
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

// A failed compound match can constrain a selected field only when every
// ancestor constructor and every sibling pattern would have matched.
fn failure_path<'pattern>(
    block: BlockId,
    mut pattern: &'pattern MatchPattern,
    subject: Place,
    path: &[Projection],
    assumptions: &[Assumption],
) -> Option<(&'pattern MatchPattern, Place, Vec<Query>)> {
    let mut requirements = Vec::new();
    let mut parent = subject;
    let mut assumptions = assumptions.to_vec();
    for projection in path {
        let mut visited = HashSet::new();
        while let MatchPattern::Alias { pattern: inner, .. } = pattern {
            if !visited.insert(pattern as *const MatchPattern) {
                return None;
            }
            pattern = inner;
        }
        let (index, fields, custom) = match (projection, pattern) {
            (Projection::Tuple(index), MatchPattern::Tuple(fields)) => (index, fields, false),
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
                    fact: Fact::Is(constructor.index),
                    assumptions: assumptions.clone(),
                });
                assumptions.push(Assumption {
                    path: parent.path.clone(),
                    constructor: constructor.index,
                });
                (index, fields, true)
            }
            _ => return None,
        };
        for (other, field) in fields.iter().enumerate() {
            if other != *index {
                let mut sibling = parent.clone();
                sibling.path.push(if custom {
                    Projection::Custom(other)
                } else {
                    Projection::Tuple(other)
                });
                // Prove each sibling on the predecessor, independently of this
                // failed match, before attributing failure to the selected field.
                pattern_requirements(block, field, sibling, &assumptions, &mut requirements)?;
            }
        }
        pattern = fields.get(*index)?;
        parent.path.push(*projection);
    }
    Some((pattern, parent, requirements))
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

impl Fact {
    fn holds(self, constructor: usize) -> bool {
        match self {
            Self::Is(expected) => constructor == expected,
            Self::IsNot(excluded) => constructor != excluded,
            Self::Boolean(_) => false,
        }
    }
}

fn match_proves(pattern: &MatchPattern, success: bool, fact: Fact) -> bool {
    if let Fact::Boolean(expected) = fact {
        return matches!(unaliased(pattern), Some(MatchPattern::Bool(value)) if (*value == expected) == success);
    }
    let Some((index, fields)) = custom(pattern) else {
        return false;
    };
    if success {
        return fact.holds(index);
    }
    matches!(fact, Fact::IsNot(excluded) if excluded == index) && fields.iter().all(irrefutable)
}

fn custom(pattern: &MatchPattern) -> Option<(usize, &[MatchPattern])> {
    match unaliased(pattern)? {
        MatchPattern::Custom {
            constructor,
            fields,
        } => Some((constructor.index, fields)),
        _ => None,
    }
}

pub(super) fn unaliased(pattern: &MatchPattern) -> Option<&MatchPattern> {
    let mut current = pattern;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current as *const MatchPattern) {
            return None;
        }
        match current {
            MatchPattern::Alias { pattern, .. } => current = pattern,
            _ => return Some(current),
        }
    }
}

pub(super) fn irrefutable(pattern: &MatchPattern) -> bool {
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
            MatchPattern::Bind(_) | MatchPattern::Discard | MatchPattern::Nil => {}
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
            MatchPattern::Bind(_) | MatchPattern::Discard | MatchPattern::Nil => continue,
            MatchPattern::Bool(value) => {
                requirements.push(Query {
                    block,
                    place,
                    fact: Fact::Boolean(*value),
                    assumptions,
                });
                continue;
            }
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
                    fact: Fact::Is(constructor.index),
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
        BlockId, Blocks, Control, CustomConstructorRefinement, Fact, Locals, MatchPattern,
        ParamLocal, ParamSlot, Types, ValueShapeDescriptor, irrefutable, match_proves,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, Edge, Jump, Match, MatchEdge, MatchEdgeArgument, MatchPatternBinding,
        ProfiledBlock, ProfiledBlockGraph, Terminator, Transfer,
    };
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::CustomConstructorId;
    use std::convert::Infallible;

    #[test]
    fn branch_exclusions_require_facts_about_the_same_tuple_projection() {
        use super::Condition;
        use crate::plan::execution::graph::{
            BlockHeader, BoolInstruction, CustomInstruction, ProfiledInstruction,
            ProfiledInstructionKind,
        };

        for (source, candidate, expected) in [
            (
                r#"
pub type Option(a) { Some(a) None }
pub type Repeat { NoRepeat ManyRepeat Many1Repeat }
fn read(repeat: Repeat, other: Repeat, default: Option(String)) -> String {
  case repeat, other, default {
    ManyRepeat, _, _ -> "many"
    Many1Repeat, _, _ -> "many1"
    NoRepeat, _, None -> "required"
    NoRepeat, _, Some(value) -> value
  }
}
pub fn main() { read(NoRepeat, ManyRepeat, Some("value")) }
"#,
                4,
                vec![(0, false), (2, false), (4, true), (5, false)],
            ),
            (
                r#"
pub type Option(a) { Some(a) None }
fn read(selected: Bool, other: Bool, default: Option(String)) -> String {
  case selected, other, default {
    True, _, _ -> "selected"
    False, _, None -> "required"
    False, _, Some(value) -> value
  }
}
pub fn main() { read(False, True, Some("value")) }
"#,
                2,
                vec![(0, false), (2, true), (3, false)],
            ),
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let original = plan.program.functions.value_returns.string_functions[1]
                .body()
                .block_graph();
            for changed_projection in [false, true] {
                let mut graph = ProfiledBlockGraph::<Infallible> {
                    entry: original.entry,
                    blocks: original
                        .blocks
                        .iter()
                        .map(|block| BlockHeader {
                            params: block.params.clone(),
                            instructions: block.instructions.clone(),
                            terminator: block.terminator.clone(),
                        })
                        .collect::<Vec<_>>()
                        .into(),
                    params: original.params.to_vec().into(),
                    instructions: original.instructions.to_vec().into(),
                };
                if changed_projection {
                    let mut instructions = graph.instructions.to_vec();
                    for instruction in
                        &mut instructions[graph.blocks[candidate].instructions.clone()]
                    {
                        if let ProfiledInstruction::Value(instruction) = instruction
                            && let ProfiledInstructionKind::Bool(BoolInstruction::TupleIndex {
                                index,
                                ..
                            })
                            | ProfiledInstructionKind::Custom(CustomInstruction::TupleIndex {
                                index,
                                ..
                            }) = &mut instruction.kind
                        {
                            *index = 1;
                        }
                    }
                    graph.instructions = instructions.into();
                }
                let blocks = Blocks::admit(&graph).unwrap();
                let control = Control::new(&blocks, &types);
                let exclusions = blocks
                    .iter()
                    .enumerate()
                    .filter_map(|(index, block)| {
                        let condition = match block.terminator() {
                            Terminator::Match(matcher) => Condition::Match {
                                matcher,
                                success: false,
                            },
                            Terminator::TestBranch(branch) => Condition::Test {
                                test: &branch.test,
                                truth: false,
                            },
                            _ => return None,
                        };
                        Some((index, control.guards.excludes(BlockId(index), condition)))
                    })
                    .collect::<Vec<_>>();
                let expected = expected
                    .iter()
                    .map(|(index, excluded)| (*index, *excluded && !changed_projection))
                    .collect::<Vec<_>>();
                assert_eq!(exclusions, expected, "{source}");
            }
        }
    }

    #[test]
    fn boolean_conditions_use_only_local_literals_and_keep_unknown_values() {
        use super::Condition;
        use crate::plan::execution::graph::{
            BoolInstruction, BoolLocalId, ProfiledInstructionKind,
        };

        let source = r#"
fn identity(value: Bool) { value }
fn read(left: Bool, right: Bool) {
  #(True == left, right == False, left == right, identity(left))
}
pub fn main() { read(False, True) }
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
        let graph = plan.program.functions.value_returns.tuple_functions[1]
            .body()
            .block_graph();
        let blocks = Blocks::admit(graph).unwrap();
        let control = Control::new(&blocks, &types);
        let comparisons = blocks
            .block(graph.entry)
            .unwrap()
            .instructions()
            .iter()
            .filter_map(|instruction| instruction.value())
            .filter_map(|instruction| match &instruction.kind {
                ProfiledInstructionKind::Bool(BoolInstruction::Test(test)) => {
                    Some((&instruction.output.local, test))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(comparisons.len(), 3);
        let called = blocks
            .block(graph.entry)
            .unwrap()
            .instructions()
            .iter()
            .filter_map(|instruction| instruction.value())
            .find(|instruction| {
                matches!(
                    instruction.kind,
                    ProfiledInstructionKind::Bool(BoolInstruction::Call { .. })
                )
            })
            .unwrap();
        for truth in [false, true] {
            for ((local, test), expected) in comparisons.iter().zip([
                Some((BoolLocalId(0), truth)),
                Some((BoolLocalId(1), !truth)),
                None,
            ]) {
                assert_eq!(
                    control.boolean_condition(graph.entry, Condition::Test { test, truth }),
                    expected
                );
                assert_eq!(
                    control.boolean_condition(BlockId(usize::MAX), Condition::Test { test, truth }),
                    None
                );
                assert!(!control.proves(graph.entry, local, Fact::Boolean(truth)));
            }
            assert!(!control.proves(graph.entry, &called.output.local, Fact::Boolean(truth)));
            assert!(!Fact::Boolean(truth).holds(0));
        }
        assert!(control.contradicts(
            graph.entry,
            Condition::Bool {
                subject: BoolLocalId(2),
                truth: false,
            }
        ));
        assert!(!control.contradicts(
            graph.entry,
            Condition::Bool {
                subject: BoolLocalId(0),
                truth: false,
            }
        ));
    }

    #[test]
    fn boolean_loops_need_entry_evidence_and_cannot_hide_a_changing_value() {
        use super::{Condition, Fact};
        use crate::plan::execution::graph::block::instruction::ProfiledValueInstruction;
        use crate::plan::execution::graph::{
            BoolBranch, BoolInstruction, BoolLocalId, BoolTest, ProfiledInstruction,
            ProfiledInstructionKind,
        };
        use crate::plan::execution::type_::ValueShapeId;

        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { True }")
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
        let shape = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| *shape == ValueShapeDescriptor::Bool)
                .unwrap(),
        );
        let input = ParamSlot::new(ParamLocal::Bool(BoolLocalId(0)), shape);
        for (initial, flip, expected) in [
            (None, false, (false, false)),
            (Some(false), false, (true, false)),
            (Some(true), false, (false, true)),
            (Some(true), true, (false, false)),
        ] {
            for matching in [false, true] {
                let (params, instructions) = if let Some(value) = initial {
                    (
                        Vec::new(),
                        vec![ProfiledInstruction::Value(ProfiledValueInstruction {
                            output: input.clone(),
                            kind: ProfiledInstructionKind::Bool(BoolInstruction::Value(value)),
                        })],
                    )
                } else {
                    (vec![input.clone()], Vec::new())
                };
                let loop_instructions = if flip {
                    vec![ProfiledInstruction::Value(ProfiledValueInstruction {
                        output: ParamSlot::new(ParamLocal::Bool(BoolLocalId(1)), shape),
                        kind: ProfiledInstructionKind::Bool(BoolInstruction::Test(BoolTest::Not(
                            BoolLocalId(0),
                        ))),
                    })]
                } else {
                    Vec::new()
                };
                let back = if flip { BoolLocalId(1) } else { BoolLocalId(0) };
                let transfer = Transfer {
                    families: Table::Static(&[]),
                };
                let graph = ProfiledBlockGraph::<Infallible>::from_parts(
                    BlockId(0),
                    vec![
                        ProfiledBlock::new(
                            params,
                            instructions,
                            Terminator::Jump(Jump {
                                edge: Edge::new(
                                    BlockId(1),
                                    vec![input.local.clone()],
                                    transfer.clone(),
                                ),
                            }),
                        ),
                        ProfiledBlock::new(
                            vec![input.clone()],
                            loop_instructions,
                            if matching {
                                Terminator::Match(Match {
                                    subject: input.local.clone(),
                                    pattern: MatchPattern::Bool(true),
                                    success: MatchEdge {
                                        target: BlockId(1),
                                        args: vec![MatchEdgeArgument::Value(ParamLocal::Bool(
                                            back,
                                        ))]
                                        .into(),
                                        bindings: Vec::new().into(),
                                        transfer: transfer.clone(),
                                    },
                                    failure: Edge::new(BlockId(2), Vec::new(), transfer),
                                })
                            } else {
                                Terminator::BoolBranch(BoolBranch {
                                    subject: BoolLocalId(0),
                                    true_: Edge::new(
                                        BlockId(1),
                                        vec![ParamLocal::Bool(back)],
                                        transfer.clone(),
                                    ),
                                    false_: Edge::new(BlockId(2), Vec::new(), transfer),
                                })
                            },
                        ),
                        ProfiledBlock::new(
                            Vec::new(),
                            Vec::new(),
                            Terminator::Exit(BlockGraphExitId(0)),
                        ),
                    ],
                );
                let blocks = Blocks::admit(&graph).unwrap();
                let control = Control::new(&blocks, &types);
                assert_eq!(
                    (
                        control.guards.excludes(
                            BlockId(1),
                            Condition::Bool {
                                subject: BoolLocalId(0),
                                truth: true
                            }
                        ),
                        control.guards.excludes(
                            BlockId(1),
                            Condition::Bool {
                                subject: BoolLocalId(0),
                                truth: false
                            }
                        ),
                    ),
                    expected
                );
                assert_eq!(
                    control.proves(BlockId(1), &input.local, Fact::Boolean(true)),
                    expected.1
                );
                assert_eq!(
                    control.proves(BlockId(1), &input.local, Fact::Boolean(false)),
                    expected.0
                );
            }
        }
    }

    #[test]
    fn boolean_patterns_keep_exact_truth_and_parent_projection_obligations() {
        use super::{Place, Projection, pattern_requirements};
        use crate::plan::execution::graph::TupleLocalId;

        for value in [false, true] {
            let pattern = MatchPattern::Alias {
                pattern: Box::new(MatchPattern::Bool(value)).into(),
                binding: MatchPatternBinding::new(0),
            };
            for success in [false, true] {
                assert!(match_proves(
                    &pattern,
                    success,
                    Fact::Boolean(value == success)
                ));
                assert!(!match_proves(
                    &pattern,
                    success,
                    Fact::Boolean(value != success)
                ));
            }
            let mut requirements = Vec::new();
            assert_eq!(
                pattern_requirements(
                    BlockId(0),
                    &MatchPattern::Tuple(vec![pattern].into()),
                    Place::local(TupleLocalId(0).into()),
                    &[],
                    &mut requirements,
                ),
                Some(())
            );
            assert_eq!(requirements.len(), 1);
            assert_eq!(
                requirements[0].place,
                Place {
                    root: TupleLocalId(0).into(),
                    path: vec![Projection::Tuple(0)],
                }
            );
            assert!(requirements[0].fact == Fact::Boolean(value));
            assert!(requirements[0].assumptions.is_empty());
        }
    }

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
        let control = Control::new(&blocks, &types);
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
                Fact::IsNot(0),
                std::slice::from_ref(&parent),
            )
            .unwrap();
        assert_eq!(obligations.len(), 1);
        assert!(obligations[0].fact == Fact::Is(0));
        assert_eq!(obligations[0].assumptions.len(), 1);
        assert!(obligations[0].assumptions[0] == parent);

        for pattern in [
            MatchPattern::Bool(true),
            MatchPattern::Custom {
                constructor: CustomConstructorId {
                    type_id: crate::plan::execution::type_::CustomTypeId(0),
                    index: 0,
                },
                fields: vec![MatchPattern::String("literal".into())].into(),
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
                        Fact::IsNot(0),
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
            assert!(query.fact == Fact::Is(constructor));
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
        for unknown in [&MatchPattern::String("literal".into()), &CYCLE] {
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
            let control = Control::new(&blocks, &types);
            assert_eq!(control.proves(BlockId(4), source, Fact::IsNot(0)), expected);
            assert_eq!(control.proves(BlockId(4), source, Fact::Is(1)), expected);
        }
    }

    #[test]
    fn requires_every_incoming_path_to_establish_a_constructor() {
        for (remainder, complete) in [
            ("Second(n) -> base + n", true),
            ("Second(_) -> base", false),
        ] {
            let source = format!(
                r#"
pub type Choice {{ First(Int) Second(Int) }}
fn read(base: Int, value: Choice) {{
  case value {{
    First(n) -> base + n
    {remainder}
  }}
}}
pub fn main() {{ read(0, First(42)) }}
"#
            );
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
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
            assert_eq!(
                common.custom_types.types[local.shape.type_id.index()]
                    .constructors
                    .iter()
                    .map(|constructor| constructor.id.index)
                    .collect::<Vec<_>>(),
                if complete { vec![0, 1] } else { vec![0] },
            );
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
                let control = Control::new(&blocks, &types);
                let parameter = &blocks.block(BlockId(1)).unwrap().params()[0];
                assert_eq!(
                    control.proves(BlockId(1), &parameter.local, Fact::Is(0)),
                    !bypass
                );
                assert_eq!(
                    control.proves(BlockId(1), &parameter.local, Fact::IsNot(1)),
                    !bypass
                );
                assert!(!control.proves(BlockId(0), &slot.local, Fact::Is(0)));
                assert_eq!(
                    control.proves(BlockId(2), &slot.local, Fact::Is(1)),
                    complete,
                );
                assert!(!control.proves(BlockId(99), &slot.local, Fact::Is(0)));
                assert!(!control.proves(
                    BlockId(1),
                    &ParamLocal::Int(crate::plan::execution::graph::IntLocalId(99)),
                    Fact::Is(0)
                ));
                let mut locals = Locals::default();
                locals.define(parameter, &types).unwrap();
                control.refine(BlockId(1), parameter, &mut locals);
                assert_eq!(locals.allows_constructor(&parameter.local, 1), bypass);
                assert!(locals.allows_constructor(&parameter.local, 0));
            }
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
            let control = Control::new(&blocks, &types);
            assert!(!control.proves(BlockId(1), &slot.local, Fact::IsNot(1)));
            let mut locals = Locals::default();
            locals.define(slot, &types).unwrap();
            control.refine(BlockId(1), slot, &mut locals);
            assert!(locals.allows_constructor(&slot.local, 0));
            assert!(locals.allows_constructor(&slot.local, 1));
            assert_eq!(locals.known_constructors().count(), 0);
        }
    }

    #[test]
    fn common_fields_refine_the_child_without_selecting_the_parent() {
        use super::{Address, Place};
        use crate::Value;

        let source = r#"
pub type Choice { First(Int) Second(Int) }
pub type Wrap { Left(inner: Choice) Right(inner: Choice) }
fn choose(value: Wrap) -> Int {
  case value.inner {
    First(_) -> 0
    _ -> {
      let assert Second(number) = value.inner
      number
    }
  }
}
pub fn main() {
  #(choose(Left(First(1))), choose(Right(First(2))),
    choose(Left(Second(3))), choose(Right(Second(4))))
}
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
        let body = plan.program.functions.value_returns.int_functions[0].body();
        let blocks = Blocks::admit(body.block_graph()).unwrap();
        let control = Control::new(&blocks, &types);
        let mut selected = 0;
        for (index, block) in blocks.iter().enumerate() {
            let block_id = BlockId(index);
            let mut locals = Locals::default();
            for slot in block.params().iter().chain(
                block
                    .instructions()
                    .iter()
                    .flat_map(|instruction| instruction.outputs()),
            ) {
                locals.define(slot, &types).unwrap();
                control.refine(block_id, slot, &mut locals);
                if control.proves(block_id, &slot.local, Fact::IsNot(0))
                    && !control.proves(block_id, &slot.local, Fact::IsNot(1))
                    && !control.proves(block_id, &slot.local, Fact::Is(1))
                {
                    assert!(!locals.allows_constructor(&slot.local, 0));
                    assert!(locals.allows_constructor(&slot.local, 1));
                    let place = Place::local(Address::of(&slot.local))
                        .normalize(block_id, &blocks)
                        .unwrap();
                    assert_eq!(
                        locals.known_constructors().collect::<Vec<_>>(),
                        vec![(&place, 1)]
                    );
                    selected += 1;
                }
            }
        }
        assert_eq!(selected, 1);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            Value::Tuple(vec![
                Value::Int(0.into()),
                Value::Int(0.into()),
                Value::Int(3.into()),
                Value::Int(4.into())
            ]),
        );
        assert!(echo.is_empty());
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
        let control = Control::new(&blocks, &types);
        assert!(control.proves(BlockId(4), &slot.local, Fact::IsNot(1)));
        assert!(control.proves(BlockId(4), &slot.local, Fact::Is(0)));
        assert!(!control.proves(BlockId(0), &slot.local, Fact::Is(0)));
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
            assert_eq!(match_proves(&pattern, false, Fact::IsNot(7)), excluded);
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
        let control = Control::new(&blocks, &types);
        let root = body.block_graph().instructions.iter().find(|instruction| {
            matches!(&instruction.value().unwrap().output.local, ParamLocal::Tuple { type_, .. } if type_.len() == 4)
        }).unwrap().value().unwrap().output.shape;
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
                matches!(&instruction.value().unwrap().output.local, ParamLocal::Custom(CustomLocal { shape, .. })
                if common.custom_types.types[shape.type_id.index()].type_.name.as_str() == "Choice")
            })
            .unwrap()
            .value().unwrap().output
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
        let control = Control::new(&blocks, &types);
        assert_eq!(
            control.projected_shape(missing_shape, &[Projection::Custom(0)], &[]),
            None
        );
    }

    #[test]
    fn failed_nested_patterns_collect_parent_and_sibling_requirements() {
        use super::{Assumption, Condition, Place, Projection, Query};
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
        let control = Control::new(&blocks, &types);
        let leaf = MatchPattern::Custom {
            constructor: CustomConstructorId {
                type_id: CustomTypeId(0),
                index: 3,
            },
            fields: Vec::new().into(),
        };
        for (custom_parent, sibling, boolean) in [
            (false, MatchPattern::Discard, false),
            (false, MatchPattern::Nil, false),
            (
                false,
                MatchPattern::Tuple(vec![MatchPattern::Nil, MatchPattern::Discard].into()),
                false,
            ),
            (false, MatchPattern::Bool(true), true),
            (true, MatchPattern::Discard, false),
            (
                true,
                MatchPattern::Alias {
                    pattern: Node::Static(&MatchPattern::Nil),
                    binding: MatchPatternBinding::new(0),
                },
                false,
            ),
            (true, MatchPattern::Bool(true), true),
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
            let requirements = control
                .condition(
                    BlockId(0),
                    Condition::Match {
                        matcher: &matcher,
                        success: false,
                    },
                    &source,
                    Fact::IsNot(3),
                    &[],
                )
                .unwrap();
            let mut expected = Vec::new();
            if custom_parent {
                expected.push(Query {
                    block: BlockId(0),
                    place: Place::local(TupleLocalId(0).into()),
                    fact: Fact::Is(7),
                    assumptions: Vec::new(),
                });
            }
            if boolean {
                expected.push(Query {
                    block: BlockId(0),
                    place: Place {
                        root: TupleLocalId(0).into(),
                        path: vec![if custom_parent {
                            Projection::Custom(1)
                        } else {
                            Projection::Tuple(1)
                        }],
                    },
                    fact: Fact::Boolean(true),
                    assumptions: if custom_parent {
                        vec![Assumption {
                            path: Vec::new(),
                            constructor: 7,
                        }]
                    } else {
                        Vec::new()
                    },
                });
            }
            assert_eq!(requirements.len(), expected.len());
            for (actual, expected) in requirements.into_iter().zip(expected) {
                assert_eq!(actual.block, expected.block);
                assert_eq!(actual.place, expected.place);
                assert!(actual.fact == expected.fact);
                assert!(actual.assumptions == expected.assumptions);
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
                        Fact::Is(3),
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
                            Fact::IsNot(3),
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
                    Fact::IsNot(3),
                    &[],
                )
                .is_none()
        );
    }

    #[test]
    fn failed_field_tests_do_not_exclude_the_outer_constructor() {
        use crate::plan::execution::graph::MatchPatternList;

        let constructor = CustomConstructorId {
            type_id: crate::plan::execution::type_::CustomTypeId(0),
            index: 7,
        };
        for (field, excluded) in [
            (MatchPattern::Discard, true),
            (MatchPattern::Bind(MatchPatternBinding { index: 0 }), true),
            (MatchPattern::Nil, true),
            (
                MatchPattern::Tuple(
                    vec![
                        MatchPattern::Nil,
                        MatchPattern::Alias {
                            pattern: Node::Static(&MatchPattern::Nil),
                            binding: MatchPatternBinding::new(0),
                        },
                    ]
                    .into(),
                ),
                true,
            ),
            (
                MatchPattern::Tuple(vec![MatchPattern::Discard, MatchPattern::Discard].into()),
                true,
            ),
            (
                MatchPattern::Tuple(vec![MatchPattern::Nil, MatchPattern::Bool(true)].into()),
                false,
            ),
            (MatchPattern::Bool(true), false),
            (MatchPattern::String("present".into()), false),
            (
                MatchPattern::List(MatchPatternList {
                    elements: Vec::new().into(),
                    tail: None,
                }),
                false,
            ),
            (
                MatchPattern::Custom {
                    constructor,
                    fields: Vec::new().into(),
                },
                false,
            ),
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
            assert!(match_proves(&pattern, true, Fact::Is(7)));
            assert!(match_proves(&pattern, true, Fact::IsNot(0)));
            assert!(!match_proves(&pattern, true, Fact::Is(0)));
            assert_eq!(match_proves(&pattern, false, Fact::IsNot(7)), excluded);
            assert!(!match_proves(&pattern, false, Fact::Is(0)));
            assert!(!match_proves(&pattern, false, Fact::IsNot(0)));
        }
        static RECURSIVE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&RECURSIVE),
            binding: MatchPatternBinding { index: 0 },
        };
        assert!(!match_proves(&RECURSIVE, true, Fact::Is(0)));
        assert!(!match_proves(&MatchPattern::Discard, false, Fact::IsNot(0)));
    }
}
