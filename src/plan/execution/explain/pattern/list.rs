use super::{write_binding, write_pattern};
use crate::plan::execution::graph::{MatchPatternList, MatchPatternListTail};

pub(super) fn write_list(output: &mut String, list: &MatchPatternList) {
    output.push('[');
    let mut separator = "";
    for pattern in list.elements() {
        output.push_str(separator);
        write_pattern(output, pattern);
        separator = ", ";
    }
    if let Some(tail) = list.tail() {
        output.push_str(separator);
        output.push_str("..");
        match tail {
            MatchPatternListTail::Ignore => output.push('_'),
            MatchPatternListTail::Bind(binding) => write_binding(output, binding),
        }
    }
    output.push(']');
}
