use super::super::super::super::value::ExplainLocal;
use crate::plan::execution::graph::BitArrayEvaluatedSize;

pub(super) fn write_evaluated_size(output: &mut String, size: &BitArrayEvaluatedSize) {
    size.value().write_local(output);
    output.push('*');
    output.push_str(&size.unit().to_string());
}
