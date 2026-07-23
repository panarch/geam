mod float;
mod int;
mod string;

pub(crate) use float::FloatSwitch;
pub(crate) use int::IntSwitch;
pub(crate) use string::StringSwitch;

use super::Edge;
use crate::plan::execution::explain::ExplainContext;

fn write_clauses<Pattern>(
    context: &mut ExplainContext<'_, '_>,
    clauses: &[(Pattern, Edge)],
    mut write_pattern: impl FnMut(&mut ExplainContext<'_, '_>, &Pattern),
) {
    context.push_str(" clauses=[");
    for (index, (pattern, edge)) in clauses.iter().enumerate() {
        if index > 0 {
            context.push_str(", ");
        }
        write_pattern(context, pattern);
        context.push_str("->");
        context.write(edge);
    }
    context.push(']');
}
