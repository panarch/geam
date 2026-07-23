use super::super::FunctionLabel;
use crate::plan::execution::ListFunctionFunctionId;

pub(super) fn list_function_function_label(function: &ListFunctionFunctionId) -> FunctionLabel {
    match function {
        ListFunctionFunctionId::Parameter { id, .. } => {
            FunctionLabel::new("function.list.parameter", id.0)
        }
        ListFunctionFunctionId::ParameterList { id, .. } => {
            FunctionLabel::new("function.list.parameter_list", id.0)
        }
        ListFunctionFunctionId::Int { id, .. } => FunctionLabel::new("function.list.int", id.0),
        ListFunctionFunctionId::String { id, .. } => {
            FunctionLabel::new("function.list.string", id.0)
        }
        ListFunctionFunctionId::BitArray { id, .. } => {
            FunctionLabel::new("function.list.bit_array", id.0)
        }
        ListFunctionFunctionId::UtfCodepoint { id, .. } => {
            FunctionLabel::new("function.list.utf_codepoint", id.0)
        }
        ListFunctionFunctionId::Custom { id, .. } => {
            FunctionLabel::new("function.list.custom", id.0)
        }
        ListFunctionFunctionId::Float { id, .. } => FunctionLabel::new("function.list.float", id.0),
        ListFunctionFunctionId::Bool { id, .. } => FunctionLabel::new("function.list.bool", id.0),
        ListFunctionFunctionId::Nil { id, .. } => FunctionLabel::new("function.list.nil", id.0),
        ListFunctionFunctionId::Tuple { id, .. } => FunctionLabel::new("function.list.tuple", id.0),
        ListFunctionFunctionId::List { id, .. } => FunctionLabel::new("function.list.list", id.0),
        ListFunctionFunctionId::Function { id, .. } => {
            FunctionLabel::new("function.list.function", id.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{FunctionFunctionId, RuntimeFunctionId};

    #[test]
    fn labels_list_returning_function_families() {
        let cases = [
            (
                "pub fn main() -> fn() -> List(value) { fn() { [] } }",
                "function.list.parameter#0",
            ),
            (
                "pub fn main() -> fn() -> List(List(value)) { fn() { [[]] } }",
                "function.list.parameter_list#0",
            ),
            (
                "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
                "function.list.int#0",
            ),
            (
                "pub fn main() -> fn() -> List(String) { fn() { [] } }",
                "function.list.string#0",
            ),
            (
                "pub fn main() -> fn() -> List(BitArray) { fn() { [] } }",
                "function.list.bit_array#0",
            ),
            (
                "pub fn main() -> fn() -> List(UtfCodepoint) { fn() { [] } }",
                "function.list.utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> fn() -> List(Boxed) { fn() { [] } }",
                "function.list.custom#0",
            ),
            (
                "pub fn main() -> fn() -> List(Float) { fn() { [] } }",
                "function.list.float#0",
            ),
            (
                "pub fn main() -> fn() -> List(Bool) { fn() { [] } }",
                "function.list.bool#0",
            ),
            (
                "pub fn main() -> fn() -> List(Nil) { fn() { [] } }",
                "function.list.nil#0",
            ),
            (
                "pub fn main() -> fn() -> List(#(Int)) { fn() { [] } }",
                "function.list.tuple#0",
            ),
            (
                "pub fn main() -> fn() -> List(List(Int)) { fn() { [] } }",
                "function.list.list#0",
            ),
            (
                "pub fn main() -> fn() -> List(fn() -> Int) { fn() { [] } }",
                "function.list.function#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::super::assert_rendered(source, expected, |plan, output| {
            let RuntimeFunctionId::Function {
                id: FunctionFunctionId::List(function),
                ..
            } = plan.main_runtime()
            else {
                panic!("source should lower a list-function-returning main function");
            };
            super::list_function_function_label(&function).push_to(output);
        });
    }

    #[test]
    #[should_panic(expected = "source should lower a list-function-returning main function")]
    fn list_function_return_shape_guard_is_visible() {
        assert_explanation("pub fn main() { 1 }", "");
    }
}
