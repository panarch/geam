use super::super::{FunctionFunctionId, ListFunctionFunctionId, ListFunctionId, RuntimeFunctionId};

#[derive(Clone, Copy)]
pub(super) struct FunctionLabel {
    family: &'static str,
    index: usize,
}

impl FunctionLabel {
    pub(super) fn new(family: &'static str, index: usize) -> Self {
        Self { family, index }
    }

    pub(super) fn push_to(self, output: &mut String) {
        output.push_str(self.family);
        output.push('#');
        output.push_str(&self.index.to_string());
    }
}

pub(super) fn runtime_function_label(function: &RuntimeFunctionId) -> FunctionLabel {
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

pub(super) fn list_function_label(function: &ListFunctionId) -> FunctionLabel {
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

pub(super) fn function_function_label(function: &FunctionFunctionId) -> FunctionLabel {
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
    use super::runtime_function_label;
    use crate::ExecutionPlan;

    #[test]
    fn labels_every_runtime_function_family() {
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
            (
                "pub fn main() -> fn() -> fn() -> Int { fn() { fn() { 1 } } }",
                "function.function#0",
            ),
        ];

        for (source, expected) in cases {
            let plan = execution_plan(source);
            let mut actual = String::new();
            runtime_function_label(&plan.main_runtime()).push_to(&mut actual);

            assert_eq!(actual, expected);
        }
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
