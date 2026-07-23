use crate::plan::execution::ConstantId;

pub(in super::super) fn write_constant<Value>(
    output: &mut String,
    family: &str,
    id: ConstantId<Value>,
) {
    output.push_str("constant.");
    output.push_str(family);
    output.push('#');
    output.push_str(&id.index().to_string());
}
