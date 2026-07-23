mod list;

use self::list::list_function_function_label;
use super::FunctionLabel;
use crate::plan::execution::FunctionFunctionId;

pub(in super::super) fn function_function_label(function: &FunctionFunctionId) -> FunctionLabel {
    match function {
        FunctionFunctionId::Generic(id) => FunctionLabel::new("function.generic", id.index()),
        FunctionFunctionId::Never(id) => FunctionLabel::new("function.never", id.index()),
        FunctionFunctionId::Int(id) => FunctionLabel::new("function.int", id.0),
        FunctionFunctionId::Float(id) => FunctionLabel::new("function.float", id.0),
        FunctionFunctionId::String(id) => FunctionLabel::new("function.string", id.0),
        FunctionFunctionId::BitArray(id) => FunctionLabel::new("function.bit_array", id.0),
        FunctionFunctionId::UtfCodepoint(id) => FunctionLabel::new("function.utf_codepoint", id.0),
        FunctionFunctionId::Custom(id) => FunctionLabel::new("function.custom", id.index()),
        FunctionFunctionId::Bool(id) => FunctionLabel::new("function.bool", id.0),
        FunctionFunctionId::Nil(id) => FunctionLabel::new("function.nil", id.0),
        FunctionFunctionId::Tuple(id) => FunctionLabel::new("function.tuple", id.0),
        FunctionFunctionId::List(id) => list_function_function_label(id),
        FunctionFunctionId::Function(id) => FunctionLabel::new("function.function", id.index()),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{ExecutionPlan, FunctionFunctionId, RuntimeFunctionId};

    #[test]
    fn labels_function_return_families() {
        let cases = [
            (
                "pub fn main() -> fn(value) -> value { fn(value) { value } }",
                "function.generic#0",
            ),
            (
                "pub fn main() -> fn() -> value { fn() { panic } }",
                "function.never#0",
            ),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                "function.int#0",
            ),
            (
                "pub fn main() -> fn() -> Float { fn() { 1.0 } }",
                "function.float#0",
            ),
            (
                "pub fn main() -> fn() -> String { fn() { \"one\" } }",
                "function.string#0",
            ),
            (
                "pub fn main() -> fn() -> BitArray { fn() { <<1>> } }",
                "function.bit_array#0",
            ),
            (
                "pub fn main() -> fn() -> UtfCodepoint { fn() { panic } }",
                "function.utf_codepoint#0",
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> fn() -> Boxed { fn() { Boxed(1) } }",
                "function.custom#0",
            ),
            (
                "pub fn main() -> fn() -> Bool { fn() { True } }",
                "function.bool#0",
            ),
            (
                "pub fn main() -> fn() -> Nil { fn() { Nil } }",
                "function.nil#0",
            ),
            (
                "pub fn main() -> fn() -> #(Int) { fn() { #(1) } }",
                "function.tuple#0",
            ),
            (
                "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
                "function.list.int#0",
            ),
            (
                "pub fn main() -> fn() -> fn() -> Int { fn() { fn() { 1 } } }",
                "function.function#0",
            ),
        ];

        for (source, expected) in cases {
            assert_explanation(source, expected);
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            super::function_function_label(&main_function_id(plan)).push_to(output);
        });
    }

    fn main_function_id(plan: &ExecutionPlan) -> FunctionFunctionId {
        let RuntimeFunctionId::Function { id, .. } = plan.main_runtime() else {
            panic!("source should lower a function-returning main function");
        };
        id
    }

    #[test]
    #[should_panic(expected = "source should lower a function-returning main function")]
    fn function_return_shape_guard_is_visible() {
        let source = "pub fn main() { 1 }";

        super::super::super::with_execution_plan(source, main_function_id);
    }
}
