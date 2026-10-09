use super::shape::CallFunction;
use super::{CallFamily, call_family, target_family};
use crate::plan::execution::compiled::CallContractTarget;
use crate::plan::execution::function::ExecutionGraphProfile;
use std::collections::{BTreeMap, BTreeSet};

/// Preparation-local membership and entry assignment. Sharing a callee does
/// not let a mixed engine replace the preferred engine for that callee's entry.
pub(super) struct CallGroup {
    pub(super) entries: BTreeSet<usize>,
    pub(super) members: BTreeSet<usize>,
    families: BTreeSet<CallFamily>,
}

impl CallGroup {
    pub(super) fn inspect<Graph: ExecutionGraphProfile>(
        functions: &[CallFunction<'_, Graph>],
    ) -> Vec<Self> {
        let targets: BTreeMap<_, _> = functions
            .iter()
            .enumerate()
            .map(|(index, function)| (function.target.key(), index))
            .collect();
        let edges: Vec<BTreeSet<usize>> = functions
            .iter()
            .map(|function| {
                let mut edges = BTreeSet::new();
                for call in &function.shape.calls {
                    match call.target {
                        CallContractTarget::Static(target) => {
                            if let Some(&index) = targets.get(&target.key())
                                && functions[index].matches_parameters(&call.args)
                            {
                                edges.insert(index);
                            }
                        }
                        _ => {
                            // Include every generated target allowed by this
                            // signature, even without a referring source creation.
                            edges.extend(functions.iter().enumerate().filter_map(
                                |(index, callee)| callee.accepts_call(call).then_some(index),
                            ));
                        }
                    }
                }
                for tail in &function.shape.tails {
                    if let Some(&index) = targets.get(&tail.target.key())
                        && functions[index].matches_parameters(&tail.args)
                    {
                        edges.insert(index);
                    }
                }
                edges
            })
            .collect();
        let mut groups: Vec<Self> = Vec::new();
        for entry in 0..functions.len() {
            let mut members = BTreeSet::from([entry]);
            let mut pending = vec![entry];
            while let Some(member) = pending.pop() {
                for &callee in &edges[member] {
                    if members.insert(callee) {
                        pending.push(callee);
                    }
                }
            }
            let families = members
                .iter()
                .flat_map(|&member| {
                    let function = &functions[member];
                    std::iter::once(target_family(function.target))
                        .chain(function.shape.calls.iter().map(call_family))
                })
                .collect();
            if let Some(group) = groups.iter_mut().find(|group| group.families == families) {
                group.entries.insert(entry);
                group.members.extend(members);
            } else {
                groups.push(Self {
                    entries: BTreeSet::from([entry]),
                    members,
                    families,
                });
            }
            // Bound code growth independently of root count. Every union remains
            // call-closed, and ties use stable function/group ordering.
            while let Some(shared) = (0..functions.len()).find(|member| {
                groups
                    .iter()
                    .filter(|group| group.members.contains(member))
                    .count()
                    > 2
            }) {
                let containing: Vec<_> = groups
                    .iter()
                    .enumerate()
                    .filter_map(|(index, group)| group.members.contains(&shared).then_some(index))
                    .collect();
                // The selected member belongs to at least three groups, so
                // the first pair exists before the stable minimum scan.
                let first = (containing[0], containing[1]);
                let key = |(left, right): (usize, usize)| {
                    let families = groups[left].families.union(&groups[right].families).count();
                    let states: usize = groups[left]
                        .members
                        .union(&groups[right].members)
                        .map(|&member| functions[member].shape.points.len())
                        .sum();
                    (
                        families
                            - groups[left]
                                .families
                                .len()
                                .max(groups[right].families.len()),
                        families,
                        states,
                        left,
                        right,
                    )
                };
                let (left, right) = containing
                    .iter()
                    .enumerate()
                    .flat_map(|(position, &left)| {
                        containing[position + 1..]
                            .iter()
                            .map(move |&right| (left, right))
                    })
                    .fold(first, |preferred, candidate| {
                        if key(candidate) < key(preferred) {
                            candidate
                        } else {
                            preferred
                        }
                    });
                let merged = groups.remove(right);
                groups[left].entries.extend(merged.entries);
                groups[left].members.extend(merged.members);
                groups[left].families.extend(merged.families);
            }
        }
        groups
    }
}

#[cfg(test)]
mod tests {
    use super::super::shape::CallProgram;
    use super::{BTreeSet, CallFamily, CallGroup};
    use crate::plan::execution::compiled::CallContractTarget;

    #[test]
    fn mixed_callers_share_callees_without_widening_their_preferred_entries() {
        let source = r#"
fn integer(value: Int) -> Int { value }
fn boolean(value: Bool) -> Bool { value }
fn text(value: String) -> String { value }
pub fn count(value: Int) -> Int { integer(value) + 1 }
pub fn mixed(value: Int) { let _ = count(value) Nil }
pub fn main() { #(mixed(7), count(7), boolean(True), text("kept")) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let groups = CallGroup::inspect(&program.functions);
        let integer = groups
            .iter()
            .find(|group| group.families == BTreeSet::from([CallFamily::Int]))
            .unwrap();
        let mixed = groups
            .iter()
            .find(|group| group.families == BTreeSet::from([CallFamily::Int, CallFamily::Nil]))
            .unwrap();
        assert!(!integer.members.is_disjoint(&mixed.members));
        assert!(integer.entries.is_disjoint(&mixed.entries));
        assert!(
            groups
                .iter()
                .any(|group| group.families == BTreeSet::from([CallFamily::Bool]))
        );
        assert!(
            groups
                .iter()
                .any(|group| group.families == BTreeSet::from([CallFamily::String]))
        );
        let entries: Vec<_> = groups
            .iter()
            .flat_map(|group| group.entries.iter().copied())
            .collect();
        assert_eq!(entries.len(), program.functions.len());
        assert_eq!(
            entries.into_iter().collect::<BTreeSet<_>>(),
            (0..program.functions.len()).collect()
        );
    }

    #[test]
    fn dynamic_targets_and_many_return_families_keep_call_closure_and_two_body_limit() {
        let source = r#"
fn first(value: Int) -> Int { value }
fn second(value: Int) -> Int { value + 1 }
fn tail(value: Int) -> Int { second(value) }
fn call(value: Int, calculate: fn(Int) -> Int) -> Int { calculate(value) }
pub fn as_bool(value: Int) -> Bool { call(value, first) == call(value, second) }
pub fn as_text(value: Int) -> String { let _ = call(value, first) "text" }
pub fn as_float(value: Int) -> Float { let _ = call(value, first) 1.0 }
pub fn as_nil(value: Int) { let _ = call(value, first) let _ = tail(value) Nil }
pub fn main() { #(as_bool(1), as_text(1), as_float(1), as_nil(1)) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let groups = CallGroup::inspect(&program.functions);
        for member in 0..program.functions.len() {
            assert!(
                (1..=2).contains(
                    &groups
                        .iter()
                        .filter(|group| group.members.contains(&member))
                        .count()
                )
            );
            assert_eq!(
                groups
                    .iter()
                    .filter(|group| group.entries.contains(&member))
                    .count(),
                1
            );
        }
        for group in &groups {
            for &member in &group.members {
                for call in &program.functions[member].shape.calls {
                    let allowed: BTreeSet<_> = program
                        .functions
                        .iter()
                        .enumerate()
                        .filter_map(|(index, callee)| match call.target {
                            CallContractTarget::Static(target) => (callee.target == target
                                && callee.matches_parameters(&call.args))
                            .then_some(index),
                            _ => callee.accepts_call(call).then_some(index),
                        })
                        .collect();
                    assert!(allowed.is_subset(&group.members));
                }
                for tail in &program.functions[member].shape.tails {
                    let allowed: BTreeSet<_> = program
                        .functions
                        .iter()
                        .enumerate()
                        .filter_map(|(index, callee)| {
                            (callee.target == tail.target && callee.matches_parameters(&tail.args))
                                .then_some(index)
                        })
                        .collect();
                    assert!(allowed.is_subset(&group.members));
                }
            }
        }
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Tuple(vec![
                crate::Value::Bool(false),
                crate::Value::String("text".into()),
                crate::Value::Float(1.0),
                crate::Value::Nil
            ])
        );
        assert!(echo.is_empty());
    }
}
