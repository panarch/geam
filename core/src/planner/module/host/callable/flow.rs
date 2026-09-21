use crate::plan::{HostFunctionTemplate, ValueShape};
use crate::planner::{HostProviderLinkReason, PlanError};
use std::collections::HashMap;

// A parameter edge records whether the target embeds its input under a type
// constructor. A cycle with an embedding edge would produce infinitely many
// specializations. Permutation, identity and closed-type cycles remain finite.
pub(super) fn validate(templates: &[&HostFunctionTemplate]) -> Result<(), PlanError> {
    let mut vertices = 0;
    let mut starts = HashMap::new();
    for (index, template) in templates.iter().enumerate() {
        starts.insert(template.id(), (vertices, index));
        vertices += template.scheme().parameters().len();
    }
    let mut outgoing = vec![Vec::new(); vertices];
    let mut incoming = vec![Vec::new(); vertices];
    let mut growing = Vec::new();
    for template in templates {
        let (source_start, _) = starts[&template.id()];
        for target in template.callable_constructions() {
            let (target_start, target_index) = starts[&target.template()];
            for (index, argument) in target.substitution().arguments().iter().enumerate() {
                let destination = target_start + index;
                let mut pending = vec![(argument, false)];
                while let Some((shape, nested)) = pending.pop() {
                    match shape {
                        ValueShape::Parameter(parameter) => {
                            let source = source_start + parameter.index();
                            outgoing[source].push(destination);
                            incoming[destination].push(source);
                            if nested {
                                growing.push((source, destination, *template, target_index));
                            }
                        }
                        ValueShape::List(item) => pending.push((item, true)),
                        ValueShape::Tuple(items) => {
                            pending.extend(items.iter().map(|item| (item, true)))
                        }
                        ValueShape::Function(function) => {
                            pending.push((function.return_shape(), true));
                            pending
                                .extend(function.argument_shapes().iter().map(|item| (item, true)));
                        }
                        ValueShape::Custom(custom) => {
                            pending.extend(custom.arguments().iter().map(|item| (item, true)))
                        }
                        ValueShape::External(external) => {
                            pending.extend(external.arguments().iter().map(|item| (item, true)))
                        }
                        ValueShape::Int
                        | ValueShape::Float
                        | ValueShape::String
                        | ValueShape::BitArray
                        | ValueShape::UtfCodepoint
                        | ValueShape::Bool
                        | ValueShape::Nil => {}
                    }
                }
            }
        }
    }
    let components = components(&outgoing, &incoming);
    for (source, destination, owner, target) in growing {
        if components[source] == components[destination] {
            let target = templates[target];
            return Err(PlanError::HostProviderLink {
                package: owner.package().clone(),
                module: owner.module().into(),
                function: owner.name().into(),
                reason: Box::new(HostProviderLinkReason::ExpandingCallableCycle {
                    package: target.package().clone(),
                    module: target.module().into(),
                    function: target.name().into(),
                }),
            });
        }
    }
    Ok(())
}

fn components(outgoing: &[Vec<usize>], incoming: &[Vec<usize>]) -> Vec<Option<usize>> {
    let mut visited = vec![false; outgoing.len()];
    let mut order = Vec::with_capacity(outgoing.len());
    for root in 0..outgoing.len() {
        if visited[root] {
            continue;
        }
        let mut pending = vec![(root, 0)];
        visited[root] = true;
        while let Some((node, offset)) = pending.last_mut() {
            if let Some(&next) = outgoing[*node].get(*offset) {
                *offset += 1;
                if !visited[next] {
                    visited[next] = true;
                    pending.push((next, 0));
                }
            } else {
                order.push(*node);
                pending.pop();
            }
        }
    }
    let mut components = vec![None; outgoing.len()];
    for root in order.into_iter().rev() {
        if components[root].is_some() {
            continue;
        }
        components[root] = Some(root);
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            for &next in &incoming[node] {
                if components[next].is_none() {
                    components[next] = Some(root);
                    pending.push(next);
                }
            }
        }
    }
    components
}
