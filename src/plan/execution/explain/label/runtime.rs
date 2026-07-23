use super::{FunctionLabel, function_function_label, list_function_label};
use crate::plan::execution::RuntimeFunctionId;

pub(in super::super) fn runtime_function_label(function: &RuntimeFunctionId) -> FunctionLabel {
    match function {
        RuntimeFunctionId::Never(id) => FunctionLabel::new("never", id.0),
        RuntimeFunctionId::Int(id) => FunctionLabel::new("int", id.0),
        RuntimeFunctionId::Float(id) => FunctionLabel::new("float", id.0),
        RuntimeFunctionId::String(id) => FunctionLabel::new("string", id.0),
        RuntimeFunctionId::BitArray(id) => FunctionLabel::new("bit_array", id.0),
        RuntimeFunctionId::UtfCodepoint(id) => FunctionLabel::new("utf_codepoint", id.0),
        RuntimeFunctionId::Custom(id) => FunctionLabel::new("custom", id.index()),
        RuntimeFunctionId::Bool(id) => FunctionLabel::new("bool", id.0),
        RuntimeFunctionId::Nil(id) => FunctionLabel::new("nil", id.0),
        RuntimeFunctionId::Tuple { id, .. } => FunctionLabel::new("tuple", id.0),
        RuntimeFunctionId::List(id) => list_function_label(id),
        RuntimeFunctionId::Function { id, .. } => function_function_label(id),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn labels_runtime_function_families() {
        let cases = [
            ("pub fn main() -> value { main() }", "never#0"),
            ("pub fn main() { 1 }", "int#0"),
            ("pub fn main() { 1.0 }", "float#0"),
            ("pub fn main() { \"one\" }", "string#0"),
            ("pub fn main() { <<1>> }", "bit_array#0"),
            (
                "pub fn main() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value }",
                "utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() { Boxed(1) }",
                "custom#0",
            ),
            ("pub fn main() { True }", "bool#0"),
            ("pub fn main() { Nil }", "nil#0"),
            ("pub fn main() { #(1) }", "tuple#0"),
            ("pub fn main() -> List(Int) { [] }", "list.int#0"),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                "function.int#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            super::runtime_function_label(&plan.main_runtime()).push_to(output);
        });
    }
}
