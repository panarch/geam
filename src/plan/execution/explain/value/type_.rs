use super::super::super::ValueType;

pub(in crate::plan::execution::explain) fn write_type(output: &mut String, type_: &ValueType) {
    match type_ {
        ValueType::Parameter(parameter) => {
            output.push_str("param#");
            output.push_str(&parameter.index().to_string());
        }
        ValueType::Int => output.push_str("Int"),
        ValueType::Float => output.push_str("Float"),
        ValueType::String => output.push_str("String"),
        ValueType::BitArray => output.push_str("BitArray"),
        ValueType::UtfCodepoint => output.push_str("UtfCodepoint"),
        ValueType::Bool => output.push_str("Bool"),
        ValueType::Nil => output.push_str("Nil"),
        ValueType::Tuple(elements) => {
            output.push_str("#(");
            for (index, element) in elements.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                write_type(output, element);
            }
            output.push(')');
        }
        ValueType::List(id) => {
            output.push_str("list_type#");
            output.push_str(&id.index().to_string());
        }
        ValueType::Function(type_) => {
            output.push_str("fn(");
            for (index, argument) in type_.argument_types().iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                write_type(output, argument);
            }
            output.push_str(") -> ");
            write_type(output, type_.return_());
        }
        ValueType::Custom(id) => {
            output.push_str("custom_type#");
            output.push_str(&id.index().to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{CustomTypeId, FunctionType, ListTypeId, ValueType};

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
            super::super::super::assert_written(expected, |output| {
                super::write_type(output, &type_);
            });
        }
    }
}
