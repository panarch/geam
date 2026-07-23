mod float;
mod int;
mod string;

pub(super) use self::float::write_float_switch;
pub(super) use self::int::write_int_switch;
pub(super) use self::string::write_string_switch;

use super::super::edge::write_edge;
use crate::plan::execution::Edge;

fn write_clauses<Pattern>(
    output: &mut String,
    clauses: &[(Pattern, Edge)],
    mut write_pattern: impl FnMut(&mut String, &Pattern),
) {
    output.push_str(" clauses=[");
    for (index, (pattern, edge)) in clauses.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_pattern(output, pattern);
        output.push_str("->");
        write_edge(output, edge);
    }
    output.push(']');
}
