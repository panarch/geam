use super::FunctionLabel;
use crate::plan::execution::ListFunctionId;

pub(in super::super) fn list_function_label(function: &ListFunctionId) -> FunctionLabel {
    match function {
        ListFunctionId::Parameter(id) => FunctionLabel::new("list.parameter", id.index()),
        ListFunctionId::ParameterList(id) => FunctionLabel::new("list.parameter_list", id.index()),
        ListFunctionId::Int(id) => FunctionLabel::new("list.int", id.index()),
        ListFunctionId::String(id) => FunctionLabel::new("list.string", id.index()),
        ListFunctionId::BitArray(id) => FunctionLabel::new("list.bit_array", id.index()),
        ListFunctionId::UtfCodepoint(id) => FunctionLabel::new("list.utf_codepoint", id.index()),
        ListFunctionId::Custom(id) => FunctionLabel::new("list.custom", id.index()),
        ListFunctionId::Float(id) => FunctionLabel::new("list.float", id.index()),
        ListFunctionId::Bool(id) => FunctionLabel::new("list.bool", id.index()),
        ListFunctionId::Nil(id) => FunctionLabel::new("list.nil", id.index()),
        ListFunctionId::Tuple(id) => FunctionLabel::new("list.tuple", id.index()),
        ListFunctionId::List(id) => FunctionLabel::new("list.list", id.index()),
        ListFunctionId::Function(id) => FunctionLabel::new("list.function", id.index()),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::RuntimeFunctionId;

    #[test]
    fn labels_list_function_families() {
        let cases = [
            ("pub fn main() -> List(value) { [] }", "list.parameter#0"),
            (
                "pub fn main() -> List(List(value)) { [[]] }",
                "list.parameter_list#0",
            ),
            ("pub fn main() -> List(Int) { [] }", "list.int#0"),
            ("pub fn main() -> List(String) { [] }", "list.string#0"),
            ("pub fn main() -> List(BitArray) { [] }", "list.bit_array#0"),
            (
                "pub fn main() -> List(UtfCodepoint) { [] }",
                "list.utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> List(Boxed) { [] }",
                "list.custom#0",
            ),
            ("pub fn main() -> List(Float) { [] }", "list.float#0"),
            ("pub fn main() -> List(Bool) { [] }", "list.bool#0"),
            ("pub fn main() -> List(Nil) { [] }", "list.nil#0"),
            ("pub fn main() -> List(#(Int)) { [] }", "list.tuple#0"),
            ("pub fn main() -> List(List(Int)) { [] }", "list.list#0"),
            (
                "pub fn main() -> List(fn() -> Int) { [] }",
                "list.function#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            let RuntimeFunctionId::List(function) = plan.main_runtime() else {
                panic!("source should lower a list-returning main function");
            };
            super::list_function_label(&function).push_to(output);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower a list-returning main function")]
    fn list_function_shape_guard_is_visible() {
        assert_explanation("pub fn main() { 1 }", "");
    }
}
