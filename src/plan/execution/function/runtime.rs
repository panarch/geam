use super::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, FunctionFunctionId,
    IntFunctionId, ListFunctionId, NeverFunctionId, NilFunctionId, StringFunctionId,
    TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::{CustomConstructorId, FunctionType, ValueShapeId, ValueType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GenericCallableId {
    Function {
        template: usize,
        substitution: Box<[ValueShapeId]>,
    },
    Constructor(CustomConstructorId),
}

impl GenericCallableId {
    pub(in crate::plan::execution) fn function(
        template: usize,
        substitution: Vec<ValueShapeId>,
    ) -> Self {
        Self::Function {
            template,
            substitution: substitution.into_boxed_slice(),
        }
    }

    pub(in crate::plan::execution) fn constructor(constructor: CustomConstructorId) -> Self {
        Self::Constructor(constructor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeFunctionId {
    Never(NeverFunctionId),
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
    Tuple {
        id: TupleFunctionId,
        return_type: Vec<ValueType>,
    },
    List(ListFunctionId),
    Function {
        id: FunctionFunctionId,
        return_type: FunctionType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionReturnFamily {
    Generic,
    Never,
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Custom,
    Bool,
    Nil,
    Tuple,
    List,
    Function,
}

impl std::fmt::Display for FunctionReturnFamily {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Generic => "Generic",
            Self::Never => "Never",
            Self::Int => "Int",
            Self::Float => "Float",
            Self::String => "String",
            Self::BitArray => "BitArray",
            Self::UtfCodepoint => "UtfCodepoint",
            Self::Custom => "Custom",
            Self::Bool => "Bool",
            Self::Nil => "Nil",
            Self::Tuple => "Tuple",
            Self::List => "List",
            Self::Function => "Function",
        })
    }
}

use crate::plan::execution::explain::FunctionLabel;
use crate::plan::execution::function::{function_function_label, list_function_label};

pub(in crate::plan::execution) fn runtime_function_label(
    function: &RuntimeFunctionId,
) -> FunctionLabel {
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
mod explain_tests {
    use crate::plan::execution::explain;

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
        explain::assert_rendered(source, expected, |plan, output| {
            super::runtime_function_label(&plan.main_runtime()).write(output);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturnFamily;

    #[test]
    fn display_names_every_family() {
        assert_eq!(
            [
                FunctionReturnFamily::Generic,
                FunctionReturnFamily::Never,
                FunctionReturnFamily::Int,
                FunctionReturnFamily::Float,
                FunctionReturnFamily::String,
                FunctionReturnFamily::BitArray,
                FunctionReturnFamily::UtfCodepoint,
                FunctionReturnFamily::Custom,
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::Nil,
                FunctionReturnFamily::Tuple,
                FunctionReturnFamily::List,
                FunctionReturnFamily::Function,
            ]
            .map(|family| family.to_string()),
            [
                "Generic",
                "Never",
                "Int",
                "Float",
                "String",
                "BitArray",
                "UtfCodepoint",
                "Custom",
                "Bool",
                "Nil",
                "Tuple",
                "List",
                "Function",
            ],
        );
    }
}
