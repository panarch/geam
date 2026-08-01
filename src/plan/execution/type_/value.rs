use super::{CustomTypeId, ExternalTypeId, FunctionType, ListTypeId};
use crate::plan;
use crate::plan::execution::explain::{Explain, ExplainContext};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueType {
    Parameter(plan::TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Vec<ValueType>),
    List(ListTypeId),
    Function(Box<FunctionType>),
    Custom(CustomTypeId),
    External(ExternalTypeId),
}

impl Explain for ValueType {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            Self::Parameter(parameter) => {
                context.push_str("param#");
                context.push_str(&parameter.index().to_string());
            }
            Self::Int => context.push_str("Int"),
            Self::Float => context.push_str("Float"),
            Self::String => context.push_str("String"),
            Self::BitArray => context.push_str("BitArray"),
            Self::UtfCodepoint => context.push_str("UtfCodepoint"),
            Self::Bool => context.push_str("Bool"),
            Self::Nil => context.push_str("Nil"),
            Self::Tuple(elements) => {
                context.push_str("#(");
                for (index, element) in elements.iter().enumerate() {
                    if index > 0 {
                        context.push_str(", ");
                    }
                    context.write(element);
                }
                context.push(')');
            }
            Self::List(id) => {
                context.push_str("list_type#");
                context.push_str(&id.index().to_string());
            }
            Self::Function(type_) => {
                context.write(type_.as_ref());
            }
            Self::Custom(id) => {
                context.push_str("custom_type#");
                context.push_str(&id.index().to_string());
            }
            Self::External(id) => {
                context.push_str("external_type#");
                context.push_str(&id.index().to_string());
            }
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::type_::{CustomTypeId, FunctionType, ListTypeId, ValueType};

    #[test]
    fn writes_primitive_and_recursive_value_types() {
        let cases = [
            (
                ValueType::Parameter(crate::plan::TypeParameterId(0)),
                "param#0",
            ),
            (ValueType::Int, "Int"),
            (ValueType::Float, "Float"),
            (ValueType::String, "String"),
            (ValueType::BitArray, "BitArray"),
            (ValueType::UtfCodepoint, "UtfCodepoint"),
            (ValueType::Bool, "Bool"),
            (ValueType::Nil, "Nil"),
            (
                ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                "#(Int, String)",
            ),
            (ValueType::List(ListTypeId::new(3)), "list_type#3"),
            (
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int, ValueType::String],
                    ValueType::Bool,
                ))),
                "fn(Int, String) -> Bool",
            ),
            (ValueType::Custom(CustomTypeId::new(4)), "custom_type#4"),
        ];

        for (type_, expected) in cases {
            explain::assert_rendered("pub fn main() { 1 }", expected, |plan, output| {
                let mut context = explain::ExplainContext::new(plan, output);
                context.write(&type_);
            });
        }
    }
}
