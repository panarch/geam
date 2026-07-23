use super::super::super::super::value::ExplainLocal;
use crate::plan::execution::graph::{BitArrayPatternSize, BitArrayPatternSizeExpr};

pub(super) fn write_size(output: &mut String, size: &BitArrayPatternSize) {
    write_expr(output, size.value());
    output.push('*');
    output.push_str(&size.unit().to_string());
}

fn write_expr(output: &mut String, expression: &BitArrayPatternSizeExpr) {
    match expression {
        BitArrayPatternSizeExpr::Value(value) => output.push_str(&value.to_string()),
        BitArrayPatternSizeExpr::Local(local) => local.write_local(output),
        BitArrayPatternSizeExpr::Binding(binding) => {
            output.push_str("binding#");
            output.push_str(&binding.index().to_string());
        }
        BitArrayPatternSizeExpr::Add { left, right } => write_binary(output, "+", left, right),
        BitArrayPatternSizeExpr::Subtract { left, right } => {
            write_binary(output, "-", left, right);
        }
        BitArrayPatternSizeExpr::Multiply { left, right } => {
            write_binary(output, "*", left, right);
        }
        BitArrayPatternSizeExpr::Divide { left, right } => write_binary(output, "/", left, right),
        BitArrayPatternSizeExpr::Remainder { left, right } => {
            write_binary(output, "%", left, right);
        }
    }
}

fn write_binary(
    output: &mut String,
    operator: &str,
    left: &BitArrayPatternSizeExpr,
    right: &BitArrayPatternSizeExpr,
) {
    output.push('(');
    write_expr(output, left);
    output.push(' ');
    output.push_str(operator);
    output.push(' ');
    write_expr(output, right);
    output.push(')');
}
