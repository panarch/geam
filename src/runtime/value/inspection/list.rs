use num_bigint::BigInt;

use super::super::list::{ListValue, ListValueKind};
use super::{bit_array, custom, function, tuple};

pub(super) fn write(output: &mut String, value: &ListValue) {
    if let ListValueKind::Int(values) = value.kind()
        && let Some(chars) = printable_charlist(values)
    {
        output.push_str("charlist.from_string(\"");
        output.push_str(&chars);
        output.push_str("\")");
        return;
    }

    match value.kind() {
        ListValueKind::Parameter(_) => output.push_str("[]"),
        ListValueKind::Int(values) => write_items(output, values, |output, value| {
            output.push_str(&value.to_string());
        }),
        ListValueKind::String(values) => write_items(output, values, |output, value| {
            output.push_str(&format!("{value:?}"));
        }),
        ListValueKind::BitArray(values) => write_items(output, values, bit_array::write),
        ListValueKind::UtfCodepoint(values) => {
            write_items(output, values, |output, value| {
                output.push_str(&format!("{value:?}"));
            });
        }
        ListValueKind::Custom { values, .. } => write_items(output, values, custom::write),
        ListValueKind::Float(values) => write_items(output, values, |output, value| {
            output.push_str(&format!("{value:?}"));
        }),
        ListValueKind::Bool(values) => write_items(output, values, |output, value| {
            output.push_str(if *value { "True" } else { "False" });
        }),
        ListValueKind::Nil(len) => {
            output.push('[');
            for index in 0..*len {
                if index != 0 {
                    output.push_str(", ");
                }
                output.push_str("Nil");
            }
            output.push(']');
        }
        ListValueKind::Tuple { values, .. } => write_items(output, values, |output, values| {
            tuple::write(output, values)
        }),
        ListValueKind::List { values, .. } => write_items(output, values, write),
        ListValueKind::Function { values, .. } => write_items(output, values, function::write),
    }
}

fn printable_charlist(values: &[BigInt]) -> Option<String> {
    if values.is_empty() {
        return None;
    }

    let mut chars = String::with_capacity(values.len());
    for value in values {
        match u8::try_from(value) {
            Ok(byte @ 32..=126) => chars.push(char::from(byte)),
            Ok(_) | Err(_) => return None,
        }
    }
    Some(chars)
}

fn write_items<T>(output: &mut String, values: &[T], mut write_item: impl FnMut(&mut String, &T)) {
    output.push('[');
    let mut separator = "";
    for value in values {
        output.push_str(separator);
        write_item(output, value);
        separator = ", ";
    }
    output.push(']');
}

#[cfg(test)]
mod tests {
    use super::super::super::{BitArrayValue, CustomValue, FunctionValue, ListValue, Value};
    use crate::plan::execution::function::{IntFunctionId, RuntimeFunctionId};
    use crate::plan::{CustomType, CustomTypeName, FunctionType, TypeParameterId, ValueType};

    #[test]
    fn writes_empty_printable_and_non_printable_int_lists() {
        let cases = [
            (
                ListValue::empty(ValueType::Parameter(TypeParameterId(0))),
                "[]",
            ),
            (ListValue::int(Vec::new()), "[]"),
            (
                ListValue::int(vec![72.into(), 105.into(), 33.into()]),
                r#"charlist.from_string("Hi!")"#,
            ),
            (
                ListValue::int(vec![31.into(), 126.into(), 256.into()]),
                "[31, 126, 256]",
            ),
        ];

        for (value, expected) in cases {
            assert_eq!(Value::List(value).inspect().to_string(), expected);
        }
    }

    #[test]
    fn writes_every_non_int_list_storage_family() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom =
            CustomValue::from_evaluated(custom_type.clone(), "Boxed".into(), 0, Vec::new());
        let function = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        );
        let cases = [
            (ListValue::string(vec!["one".into()]), r#"["one"]"#),
            (
                ListValue::bit_array(vec![BitArrayValue::from_bytes(vec![1])]),
                "[<<1>>]",
            ),
            (ListValue::utf_codepoint(vec!['A']), "['A']"),
            (
                ListValue::from_evaluated_custom(custom_type, vec![custom]),
                "[Boxed]",
            ),
            (ListValue::float(vec![1.5]), "[1.5]"),
            (ListValue::bool(vec![true, false]), "[True, False]"),
            (ListValue::nil(2), "[Nil, Nil]"),
            (
                ListValue::from_evaluated_tuple(
                    vec![ValueType::Int],
                    vec![vec![Value::Int(1.into())]],
                ),
                "[#(1)]",
            ),
            (
                ListValue::from_evaluated_list(
                    ValueType::Int,
                    vec![ListValue::int(vec![1.into()])],
                ),
                "[[1]]",
            ),
            (
                ListValue::from_evaluated_function(function.type_(), vec![function]),
                "[//fn() { ... }]",
            ),
        ];

        for (value, expected) in cases {
            assert_eq!(Value::List(value).inspect().to_string(), expected);
        }
    }
}
